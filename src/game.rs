use bevy::prelude::*;
use crate::despawn_screen;
use crate::enums::game::GameState;
use crate::gizmo::Movable;
use crate::player::{Player, PlayerPlugin, Vitals};
use crate::ui::GameUI;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Game),game_setup)
            .add_plugins((PlayerPlugin, GameUI))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

fn game_setup(mut commands: Commands, asset_server: ResMut<AssetServer>){
    let player_sprite = asset_server.load("assets/sprites/placeholder.png");

    commands.spawn(
        (
            Player,
            Movable{ accel: 0.0, speed_x: 0.0, speed_y: 0.0 },
            Vitals{ vitals_timer: Default::default(), health: 100, oxygen: 100.0, hydrogen: 100.0},
            OnGameScreen,
            ImageBundle {
                image: UiImage::new(player_sprite),
                ..default()
            },
        )
    );
}