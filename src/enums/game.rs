use bevy::prelude::Resource;

#[derive(Resource)]
enum GameState {
    Splash,
    Menu,
    Game
}