mod menu;

use bevy::app::App;
use bevy::prelude::*;
use menu::MenuPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins(MenuPlugin);
    }
}
