use bevy::prelude::*;
use crate::gizmos::gizmo::*;

#[derive(Component)]
pub struct Player {
    pub vitals_timer: Timer,
}

#[derive(Resource)]
pub struct Vitals {
    pub health: i32,
    pub oxygen: f32,
    pub hydrogen: f32,
}

pub fn player_movement(
    mut characters: Query<(&mut Transform, &mut Movable), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut vitals: ResMut<Vitals>,
) {
    for (mut transform, mut player) in &mut characters {
        if vitals.hydrogen >= 0.0 {
            if input.pressed(KeyCode::W) {
                player.speed_y += player.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::S) {
                player.speed_y -= player.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::D) {
                player.speed_x += player.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::A) {
                player.speed_x -= player.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
        }
        move_gizmo(&mut transform, &player);
    }
}

pub fn vitals_tick(
    time: Res<Time>,
    mut vitals: ResMut<Vitals>,
    mut characters: Query<&mut Player>,
){
    for mut player in &mut characters {
        player.vitals_timer.tick(time.delta());
        if player.vitals_timer.finished(){
            vitals.oxygen -= 0.5;
            vitals.hydrogen += 0.5;
            info!("Oxygen now at {}", vitals.oxygen);
            info!("Hydrogen at {}", vitals.hydrogen);
        }
        if vitals.oxygen <= 0.0 {
            panic!("No Oxygen!") //will be improved upon once I figure out how to make the space station
        }
    }
}