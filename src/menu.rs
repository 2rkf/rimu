use crate::GameState;

use bevy::prelude::*;

#[derive(Resource)]
struct MenuData {
    root_entities: Vec<Entity>,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(Update, handle_settings_interaction.run_if(in_state(GameState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let version = env!("CARGO_PKG_VERSION");

    commands.spawn((
        Transform::from_scale(Vec3::splat(1.5)),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    ));

    // Title "Rimu!"
    let title_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(-20.0), Val::Px(0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Rimu!"),
                TextFont {
                    font: asset_server.load("fonts/neon.otf"),
                    font_size: 75.0,
                    ..Default::default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(Color::srgb(1.0, 0.75, 0.8)),
            ));

            parent.spawn((
                Text::new(format!("v{}", version)),
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                TextFont {
                    font_size: 15.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(15.0), Val::Px(0.0)),
                    ..Default::default()
                },
            ));
        })
        .id();

    let touch_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(20.0), Val::Px(0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            // Touch to Play
            parent.spawn((
                Text::new("Touch to Play"),
                TextFont {
                    font: asset_server.load("fonts/neon_club.otf"),
                    font_size: 15.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(100.0), Val::Px(0.0)),
                    ..Default::default()
                }
            ));
        })
        .id();

    // Settings
    let settings_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexEnd,
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::Px(0.0), Val::Px(10.0), Val::Px(0.0), Val::Px(0.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                Button,
                ImageNode {
                    image: asset_server.load("icons/settings.png"),
                    ..Default::default()
                },
                Node {
                    width: Val::Px(75.0),
                    height: Val::Px(75.0),
                    ..Default::default()
                },
                Interaction::default()
            ));
        })
        .id();

        commands.insert_resource(MenuData {
            root_entities: vec![
                title_entity,
                touch_entity,
                settings_entity,
            ]
        });
}

fn handle_settings_interaction(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Settings);
            commands.spawn(AudioPlayer::new(asset_server.load("audio/button.ogg")));
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    for entity in &menu_data.root_entities {
        commands.entity(*entity).despawn_recursive();
    }
}
