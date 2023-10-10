use bevy::prelude::{Component, Resource, States};
use bevy::reflect::Reflect;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
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