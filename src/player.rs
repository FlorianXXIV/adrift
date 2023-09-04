use bevy::prelude::*;
#[derive(Component)]
pub struct Player {
    pub accel: f32,
    pub speed_x: f32,
    pub speed_y: f32,
}
pub fn character_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        if input.pressed(KeyCode::W) {
            player.speed_y += player.accel * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            player.speed_y -= player.accel * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            player.speed_x += player.accel * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            player.speed_x -= player.accel * time.delta_seconds();
        }
        if player.speed_x.abs() >= 0.5 {
            transform.translation.x += player.speed_x;
        }
        if player.speed_y.abs() >= 0.5 {
            transform.translation.y += player.speed_y;
        }
    }
}