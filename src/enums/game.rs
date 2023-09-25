use bevy::prelude::{Component, Resource};
use bevy::reflect::Reflect;

#[derive(Resource)]
enum GameState {
    Splash,
    Menu,
    Game
}

#[derive(Component, Default, Reflect)]
pub enum ResourceType{
    #[default]
    IRON,
    COPPER,
    ICE,
    HYDROGEN,
    OXYGEN,
}