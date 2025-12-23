//! UI Systems - Menus and HUD

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::states::GameState;
use crate::shared::domain::PlayerClass;

// ============ Color Constants ============

const DARK_BG: Color = Color::srgb(0.039, 0.039, 0.039);      // #0a0a0a
const DARK_PANEL: Color = Color::srgb(0.102, 0.102, 0.180);   // #1a1a2e
const BLOOD_RED: Color = Color::srgb(0.545, 0.0, 0.0);        // #8b0000
const MAGIC_PURPLE: Color = Color::srgb(0.290, 0.0, 0.502);   // #4a0080
const GOLD: Color = Color::srgb(0.855, 0.647, 0.125);         // #daa520
const TEXT_WHITE: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_GRAY: Color = Color::srgb(0.6, 0.6, 0.6);

// ============ Main Menu ============

pub fn spawn_main_menu(mut commands: Commands) {
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
                Text::new("어둠의전설 M"),
                TextFont {
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
                Text::new("Legend of Darkness"),
                TextFont {
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
            spawn_menu_button(parent, "게임 시작", ButtonAction::CharacterSelect);
            
            // Quit Button
            spawn_menu_button(parent, "종료", ButtonAction::Quit);
        });
}

fn spawn_menu_button(parent: &mut ChildBuilder, text: &str, action: ButtonAction) {
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
                Text::new(text),
                TextFont {
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
                Text::new("직업 선택"),
                TextFont {
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
                spawn_class_button(row, "전사", PlayerClass::Warrior);
                spawn_class_button(row, "도적", PlayerClass::Rogue);
                spawn_class_button(row, "마법사", PlayerClass::Mage);
                spawn_class_button(row, "성직자", PlayerClass::Cleric);
                spawn_class_button(row, "무도가", PlayerClass::MartialArtist);
            });
            
            // Confirm button
            spawn_menu_button(parent, "확인", ButtonAction::ConfirmCharacter);
            
            // Back button
            spawn_menu_button(parent, "뒤로", ButtonAction::BackToMenu);
        });
}

fn spawn_class_button(parent: &mut ChildBuilder, text: &str, class: PlayerClass) {
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
                Text::new(text),
                TextFont {
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
    player_query: Query<&PlayerStats, With<PlayerComponent>>,
    mut hp_bar_query: Query<&mut Node, (With<HpBar>, Without<MpBar>, Without<ExpBar>)>,
    mut mp_bar_query: Query<&mut Node, (With<MpBar>, Without<HpBar>, Without<ExpBar>)>,
    mut level_text_query: Query<&mut Text, (With<LevelText>, Without<GoldText>)>,
    mut gold_text_query: Query<&mut Text, (With<GoldText>, Without<LevelText>)>,
) {
    if let Ok(stats) = player_query.get_single() {
        // Update HP bar
        if let Ok(mut node) = hp_bar_query.get_single_mut() {
            let hp_percent = (stats.hp as f32 / stats.max_hp as f32) * 100.0;
            node.width = Val::Percent(hp_percent);
        }
        
        // Update MP bar
        if let Ok(mut node) = mp_bar_query.get_single_mut() {
            let mp_percent = (stats.mp as f32 / stats.max_mp as f32) * 100.0;
            node.width = Val::Percent(mp_percent);
        }
        
        // Update level text
        if let Ok(mut text) = level_text_query.get_single_mut() {
            **text = format!("Lv.{}", stats.level);
        }
        
        // Update gold text
        if let Ok(mut text) = gold_text_query.get_single_mut() {
            **text = format!("💰 {}", stats.gold);
        }
    }
}
