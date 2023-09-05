use bevy::prelude::Resource;

#[derive(Resource)]
pub enum DisplayQuality {
    LOW,
    MEDIUM,
    HIGH
}
