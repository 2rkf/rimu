use crate::GameState;
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
            .add_systems(Update, handle_settings_interaction.run_if(in_state(GameState::Settings)))
            .add_systems(OnExit(GameState::Settings), cleanup_settings_menu);
    }
}

#[derive(Component)]
struct SettingsMenu;

fn setup_settings_menu(mut commands: Commands, res: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
        },
        SettingsMenu,
    ))
    .with_children(|parent| {
        parent.spawn((
            ImageNode {
                image: res.load("icons/back.png"),
                ..Default::default()
            },
            Node {
                width: Val::Px(25.0),
                height: Val::Px(25.0),
                margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(22.5), Val::Px(0.0)),
                ..Default::default()
            },
            Button,
        ));

        parent.spawn((
            Text::new("Settings"),
            Node {
                margin: UiRect::new(Val::Px(10.0), Val::Px(0.0), Val::Px(20.0), Val::Px(0.0)),
                ..Default::default()
            },
            TextFont {
                font_size: 25.0,
                ..Default::default()
            },
            TextColor(Color::WHITE)
        ));
    });
}

fn handle_settings_interaction(
    mut interaction_query: Query<&Interaction>,
    mut game_state: ResMut<NextState<GameState>>
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
        }
    }
}

fn cleanup_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
