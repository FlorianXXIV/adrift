use bevy::prelude::{Component, Resource};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    LOW,
    MEDIUM,
    HIGH
}
