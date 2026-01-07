use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameResources::default());
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_ui);
    }
}

#[derive(Resource, Default)]
pub struct GameResources {
    pub wood: u32,
    pub gold: u32,
}

#[derive(Component)]
struct ResourceText;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Wood: 0 | Gold: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ResourceText,
            ));
        });
}

fn update_ui(
    resources: Res<GameResources>,
    mut query: Query<&mut Text, With<ResourceText>>,
) {
    for mut text in &mut query {
        text.0 = format!("Wood: {} | Gold: {}", resources.wood, resources.gold);
    }
}
