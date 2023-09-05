use bevy::prelude::*;

pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Movable>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Movable {
    pub accel: f32,
    pub speed_x: f32,
    pub speed_y: f32,
}

pub fn move_gizmo (transform: &mut Mut<Transform>, movable: &Mut<Movable>) {
    if movable.speed_x.abs() >= 0.5 {
        transform.translation.x += movable.speed_x;
    }
    if movable.speed_y.abs() >= 0.5 {
        transform.translation.y += movable.speed_y;
    }
}