//! UI Systems - Menus and HUD

use bevy::prelude::*;
use super::components::*;
use super::resources::*;
use super::states::GameState;
use crate::shared::domain::PlayerClass;
use crate::shared::domain::character::models::Player;

// ============ Color Constants ============

const DARK_BG: Color = Color::srgb(0.05, 0.05, 0.08);         // #0d0d14
const DARK_PANEL: Color = Color::srgb(0.15, 0.15, 0.25);      // #262640
const BLOOD_RED: Color = Color::srgb(0.8, 0.2, 0.2);          // #cc3333
const MAGIC_PURPLE: Color = Color::srgb(0.4, 0.2, 0.8);       // #6633cc
const GOLD: Color = Color::srgb(1.0, 0.8, 0.2);               // #ffcc33
const TEXT_WHITE: Color = Color::srgb(0.95, 0.95, 0.95);
const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.35);
const BUTTON_HOVER: Color = Color::srgb(0.3, 0.3, 0.5);
const BUTTON_PRESSED: Color = Color::srgb(0.1, 0.1, 0.2);

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
            ZIndex(100), // Ensure it's on top
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title Layer
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(80.0)),
                    ..default()
                },
            )).with_children(|title_box| {
                // Main Title
                title_box.spawn((
                    Text::new("Legend of Darkness"),
                    TextFont {
                        font: assets.ui_font.clone(),
                        font_size: 64.0, // Mobile friendly large text
                        ..default()
                    },
                    TextColor(BLOOD_RED),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
                
                // Subtitle
                title_box.spawn((
                    Text::new("M O B I L E"),
                    TextFont {
                        font: assets.ui_font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(MAGIC_PURPLE),
                ));
            });
            
            // Menu Buttons
            let button_style = Node {
                width: Val::Px(280.0),
                height: Val::Px(70.0),
                margin: UiRect::all(Val::Px(15.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };

            // Start Game Button
            spawn_styled_button(
                parent, 
                text.get("ui.start_game"), 
                ButtonAction::CharacterSelect, 
                assets.ui_font.clone(),
                button_style.clone()
            );

            // Quit Button
            spawn_styled_button(
                parent, 
                text.get("ui.quit"), 
                ButtonAction::Quit, 
                assets.ui_font.clone(),
                button_style
            );
        });
}

fn spawn_styled_button(
    parent: &mut ChildBuilder, 
    text: &str, 
    action: ButtonAction, 
    font: Handle<Font>,
    style: Node
) {
    parent
        .spawn((
            Button,
            style,
            BackgroundColor(BUTTON_NORMAL),
            BorderRadius::all(Val::Px(15.0)), // Rounded corners
            action,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font,
                    font_size: 32.0, // Large readable font
                    ..default()
                },
                TextColor(TEXT_WHITE),
            ));
        });
}

pub fn main_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor, &mut Node),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, action, mut bg_color, mut node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(BUTTON_PRESSED);
                node.top = Val::Px(2.0); // Simple click effect
                
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
                *bg_color = BackgroundColor(BUTTON_HOVER);
                node.top = Val::Px(0.0);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(BUTTON_NORMAL);
                node.top = Val::Px(0.0);
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
    // Set default selection
    if selected_class.class.is_none() {
        selected_class.class = Some(PlayerClass::Warrior);
        selected_class.gender = "male".to_string();
        selected_class.username = "Player".to_string();
    }
    
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(DARK_BG),
            ZIndex(100),
            CharacterSelectUI,
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new(text.get("ui.character_select")),
                TextFont {
                    font: assets.ui_font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(GOLD),
                Node {
                    margin: UiRect::vertical(Val::Px(30.0)),
                    ..default()
                },
            ));
            
            // Class Grid
            parent.spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); 3], // 3 columns
                    grid_template_rows: vec![GridTrack::auto(); 2],    // 2 rows
                    row_gap: Val::Px(20.0),
                    column_gap: Val::Px(20.0),
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ))
            .with_children(|grid| {
                spawn_class_button(grid, text.get("ui.warrior"), PlayerClass::Warrior, assets.ui_font.clone(), &selected_class);
                spawn_class_button(grid, text.get("ui.rogue"), PlayerClass::Rogue, assets.ui_font.clone(), &selected_class);
                spawn_class_button(grid, text.get("ui.mage"), PlayerClass::Mage, assets.ui_font.clone(), &selected_class);
                spawn_class_button(grid, text.get("ui.cleric"), PlayerClass::Cleric, assets.ui_font.clone(), &selected_class);
                spawn_class_button(grid, text.get("ui.martial_artist"), PlayerClass::MartialArtist, assets.ui_font.clone(), &selected_class);
            });
            
            // Footer Actions
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
            )).with_children(|footer| {
                // Back Button
                spawn_styled_button(
                    footer, 
                    text.get("ui.back"), 
                    ButtonAction::BackToMenu, 
                    assets.ui_font.clone(),
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }
                );
                
                // Confirm Button
                spawn_styled_button(
                    footer, 
                    text.get("ui.select"), 
                    ButtonAction::ConfirmCharacter, 
                    assets.ui_font.clone(),
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }
                );
            });
        });
}

fn spawn_class_button(
    parent: &mut ChildBuilder, 
    text: &str, 
    class: PlayerClass, 
    font: Handle<Font>,
    selected: &SelectedClass
) {
    let is_selected = Some(class) == selected.class;
    let border_color = if is_selected { GOLD } else { MAGIC_PURPLE };
    let bg_color = if is_selected { BUTTON_HOVER } else { DARK_PANEL };

    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(if is_selected { 3.0 } else { 1.0 })),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(10.0)),
            ButtonAction::SelectClass(class),
        ))
        .with_children(|btn| {
            // Icon (Placeholder)
            btn.spawn((
                Text::new(match class {
                    PlayerClass::Warrior => "⚔️",
                    PlayerClass::Rogue => "🗡️",
                    PlayerClass::Mage => "🔮",
                    PlayerClass::Cleric => "✝️",
                    PlayerClass::MartialArtist => "🥊",
                }),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                }
            ));

            // Label
            btn.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font,
                    font_size: 16.0,
                    ..default()
                },
                TextColor(TEXT_WHITE),
            ));
        });
}

pub fn character_select_interaction(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor, &mut Node),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut selected_class: ResMut<SelectedClass>,
) {
    for (interaction, action, mut bg_color, mut node) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // Button press effect
                node.top = Val::Px(2.0);
                *bg_color = BackgroundColor(BUTTON_PRESSED);

                match action {
                    ButtonAction::SelectClass(class) => {
                        selected_class.class = Some(*class);
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
                node.top = Val::Px(0.0);
                // Hover color logic is handled in visual system for class buttons
                 if !matches!(action, ButtonAction::SelectClass(_)) {
                    *bg_color = BackgroundColor(BUTTON_HOVER);
                }
            }
            Interaction::None => {
                node.top = Val::Px(0.0);
                 if !matches!(action, ButtonAction::SelectClass(_)) {
                    *bg_color = BackgroundColor(BUTTON_NORMAL);
                }
            }
        }
    }
}

pub fn update_character_select_visuals(
    selected_class: Res<SelectedClass>,
    mut button_query: Query<(&ButtonAction, &mut BorderColor, &mut BackgroundColor, &Interaction), With<Button>>,
) {
    if !selected_class.is_changed() {
        return;
    }

    for (action, mut border, mut bg, interaction) in &mut button_query {
        if let ButtonAction::SelectClass(class) = action {
            if Some(*class) == selected_class.class {
                *border = BorderColor(GOLD);
                if *interaction != Interaction::Pressed {
                    *bg = BackgroundColor(BUTTON_HOVER);
                }
            } else {
                *border = BorderColor(MAGIC_PURPLE);
                if *interaction == Interaction::None {
                    *bg = BackgroundColor(DARK_PANEL);
                }
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
            node.width = Val::Percent(hp_percent.clamp(0.0, 100.0));
        }
        
        // Update MP bar
        if let Ok(mut node) = mp_bar_query.get_single_mut() {
            let mp_percent = (player.combat_stats.mp as f32 / player.combat_stats.max_mp as f32) * 100.0;
            node.width = Val::Percent(mp_percent.clamp(0.0, 100.0));
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
