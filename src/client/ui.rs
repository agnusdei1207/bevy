//! UI Systems - Menus and HUD

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::states::GameState;
use crate::shared::domain::PlayerClass;
use crate::shared::domain::character::models::Player;

// ============ Color Constants ============

const DARK_BG: Color = Color::srgb(0.039, 0.039, 0.039);      // #0a0a0a
const DARK_PANEL: Color = Color::srgb(0.102, 0.102, 0.180);   // #1a1a2e
const BLOOD_RED: Color = Color::srgb(0.545, 0.0, 0.0);        // #8b0000
const MAGIC_PURPLE: Color = Color::srgb(0.290, 0.0, 0.502);   // #4a0080
const GOLD: Color = Color::srgb(0.855, 0.647, 0.125);         // #daa520
const TEXT_WHITE: Color = Color::srgb(0.9, 0.9, 0.9);

// ============ Main Menu ============

pub fn spawn_main_menu(mut commands: Commands, text: Res<TextResource>, assets: Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(DARK_BG),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Legend of Darkness"),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 72.0,
                    ..default()
                },
                TextColor(BLOOD_RED),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Subtitle
            parent.spawn((
                Text::new("Dark Fantasy RPG"),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(MAGIC_PURPLE),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));
            
            // Start Game Button
            spawn_menu_button(parent, text.get("ui.start_game"), ButtonAction::CharacterSelect, assets.ui_font.clone());

            // Quit Button
            spawn_menu_button(parent, text.get("ui.quit"), ButtonAction::Quit, assets.ui_font.clone());
        });
}

fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction, font: Handle<Font>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(DARK_PANEL),
            BorderColor(MAGIC_PURPLE),
            action,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(TEXT_WHITE),
            ));
        });
}

pub fn main_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, action, mut bg_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    ButtonAction::CharacterSelect => {
                        next_state.set(GameState::CharacterSelect);
                    }
                    ButtonAction::Quit => {
                        exit.send(AppExit::Success);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BLOOD_RED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(DARK_PANEL);
            }
        }
    }
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============ Character Select ============

pub fn spawn_character_select(
    mut commands: Commands,
    mut selected_class: ResMut<SelectedClass>,
    text: Res<TextResource>,
    assets: Res<GameAssets>,
) {
    selected_class.class = Some(PlayerClass::Warrior);
    selected_class.gender = "male".to_string();
    selected_class.username = "Player".to_string();
    
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(DARK_BG),
            CharacterSelectUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new(text.get("ui.character_select")),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(GOLD),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));
            
            // Class buttons container
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ))
            .with_children(|row| {
                spawn_class_button(row, text.get("ui.warrior"), PlayerClass::Warrior, assets.ui_font.clone());
                spawn_class_button(row, text.get("ui.rogue"), PlayerClass::Rogue, assets.ui_font.clone());
                spawn_class_button(row, text.get("ui.mage"), PlayerClass::Mage, assets.ui_font.clone());
                spawn_class_button(row, text.get("ui.cleric"), PlayerClass::Cleric, assets.ui_font.clone());
                spawn_class_button(row, text.get("ui.martial_artist"), PlayerClass::MartialArtist, assets.ui_font.clone());
            });
            
            // Confirm button
            spawn_menu_button(parent, text.get("ui.select"), ButtonAction::ConfirmCharacter, assets.ui_font.clone());
            
            // Back button
            spawn_menu_button(parent, text.get("ui.back"), ButtonAction::BackToMenu, assets.ui_font.clone());
        });
}

fn spawn_class_button(parent: &mut ChildBuilder, text: &str, class: PlayerClass, font: Handle<Font>) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(120.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(DARK_PANEL),
            BorderColor(MAGIC_PURPLE),
            ButtonAction::SelectClass(class),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font,
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_WHITE),
            ));
        });
}

pub fn character_select_interaction(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut selected_class: ResMut<SelectedClass>,
) {
    for (interaction, action, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    ButtonAction::SelectClass(class) => {
                        selected_class.class = Some(*class);
                        *border_color = BorderColor(GOLD);
                    }
                    ButtonAction::ConfirmCharacter => {
                        if selected_class.class.is_some() {
                            next_state.set(GameState::Playing);
                        }
                    }
                    ButtonAction::BackToMenu => {
                        next_state.set(GameState::MainMenu);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BLOOD_RED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(DARK_PANEL);
            }
        }
    }
}

pub fn cleanup_character_select(
    mut commands: Commands,
    query: Query<Entity, With<CharacterSelectUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ============ HUD ============

pub fn update_hud(
    player_query: Query<&Player, With<PlayerComponent>>,
    mut hp_bar_query: Query<&mut Node, (With<HpBar>, Without<MpBar>, Without<ExpBar>)>,
    mut mp_bar_query: Query<&mut Node, (With<MpBar>, Without<HpBar>, Without<ExpBar>)>,
    mut level_text_query: Query<&mut Text, (With<LevelText>, Without<GoldText>)>,
    mut gold_text_query: Query<&mut Text, (With<GoldText>, Without<LevelText>)>,
) {
    if let Ok(player) = player_query.get_single() {
        // Update HP bar
        if let Ok(mut node) = hp_bar_query.get_single_mut() {
            let hp_percent = (player.combat_stats.hp as f32 / player.combat_stats.max_hp as f32) * 100.0;
            node.width = Val::Percent(hp_percent);
        }
        
        // Update MP bar
        if let Ok(mut node) = mp_bar_query.get_single_mut() {
            let mp_percent = (player.combat_stats.mp as f32 / player.combat_stats.max_mp as f32) * 100.0;
            node.width = Val::Percent(mp_percent);
        }
        
        // Update level text
        if let Ok(mut text) = level_text_query.get_single_mut() {
            **text = format!("Lv.{}", player.level);
        }
        
        // Update gold text
        if let Ok(mut text) = gold_text_query.get_single_mut() {
            **text = format!("Gold: {}", player.gold);
        }
    }
}
