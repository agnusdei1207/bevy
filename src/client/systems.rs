//! Core Game Systems

use bevy::prelude::*;
use super::resources::*;
use super::states::GameState;

/// Setup 2D camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

use bevy::tasks::{IoTaskPool, Task};
use futures_lite::future;
use crate::shared::domain::skill::models::Skill;

/// Loading task for skills
#[derive(Component)]
pub struct SkillFetchTask(pub Task<Result<Vec<Skill>, String>>);

/// Loading task for i18n
#[derive(Component)]
pub struct I18nFetchTask(pub Task<Result<(String, serde_json::Value), String>>);

/// Load game assets
pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    i18n: Res<I18nResource>,
) {
    // Load default font (NanumGothic supports CJK characters)
    game_assets.ui_font = asset_server.load("fonts/NanumGothic.ttf");
    
    let thread_pool = IoTaskPool::get();
    
    // Start fetching skills from DB/API
    let skill_task = thread_pool.spawn(async move {
        crate::shared::domain::skill::server::get_skills(None, 99).await
    });
    commands.spawn(SkillFetchTask(skill_task));

    // Start fetching i18n pack
    let lang = i18n.current_lang.clone();
    let i18n_task = thread_pool.spawn(async move {
        let base_url = if cfg!(target_arch = "wasm32") {
            "/assets/i18n"
        } else {
            "http://localhost:3000/assets/i18n"
        };
        let url = format!("{}/{}.json", base_url, lang);
        
        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let pack = response.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;
        Ok((lang, pack))
    });
    commands.spawn(I18nFetchTask(i18n_task));

    game_assets.assets_loaded = true;
}

/// Check if assets and skills are loaded
pub fn check_assets_loaded(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut skill_data: ResMut<SkillData>,
    mut i18n: ResMut<I18nResource>,
    mut skill_tasks: Query<(Entity, &mut SkillFetchTask)>,
    mut i18n_tasks: Query<(Entity, &mut I18nFetchTask)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut skills_done = false;
    for (entity, mut task) in &mut skill_tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            match result {
                Ok(skills) => {
                    skill_data.skills = skills;
                    println!("✅ Skills loaded from DB: {}", skill_data.skills.len());
                }
                Err(e) => {
                    eprintln!("❌ Failed to load skills: {}", e);
                }
            }
            commands.entity(entity).despawn();
            skills_done = true;
        }
    }

    let mut i18n_done = false;
    for (entity, mut task) in &mut i18n_tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            match result {
                Ok((lang, pack)) => {
                    i18n.current_lang = lang;
                    i18n.pack = pack;
                    println!("✅ I18n pack loaded: {}", i18n.current_lang);
                }
                Err(e) => {
                    eprintln!("❌ Failed to load i18n: {}", e);
                }
            }
            commands.entity(entity).despawn();
            i18n_done = true;
        }
    }

    let all_done = game_assets.assets_loaded 
        && (skills_done || !skill_data.skills.is_empty())
        && (i18n_done || !i18n.pack.as_object().unwrap().is_empty());

    if all_done {
        next_state.set(GameState::MainMenu);
    }
}
