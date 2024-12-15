use crate::GameState;

use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, handle_touch);
    }
}

fn setup_menu(mut commands: Commands, res: Res<AssetServer>) {
    let version = env!("CARGO_PKG_VERSION");

    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_scale(Vec3::splat(1.5)),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1))
    ));

    // Title "Rimu!"
    commands
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
                    font: res.load("neon.otf"),
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
                    font_size: 20.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(15.0), Val::Px(0.0)),
                    ..Default::default()
                }
            ));
        });

        commands.spawn(Node {
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
                    font: res.load("neon_club.otf"),
                    font_size: 15.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(0.0)),
                    ..Default::default()
                }
            ));
        });

        // Settings
        commands.spawn(Node {
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
                ImageNode {
                    image: res.load("settings.png"),
                    ..Default::default()
                },
                Node {
                    width: Val::Px(75.0),
                    height: Val::Px(75.0),
                    ..Default::default()
                },
                Interaction::default()
            ));
        });
}

fn handle_touch( 
    mut app_exit_events: EventWriter<AppExit>,
    touches: Res<Touches>,
) {
    if touches.just_pressed(0) {
        println!("Screen touched! Starting the game...");
        app_exit_events.send(AppExit::Success);
    }
}
