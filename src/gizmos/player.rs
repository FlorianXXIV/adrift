use bevy::prelude::*;
use crate::gizmos::gizmo::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (player_movement, vitals_tick))
            .register_type::<Vitals>();
    }
}

#[derive(Component)]
pub struct Player;
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Vitals {
    pub vitals_timer: Timer,
    pub health: i32,
    pub oxygen: f32,
    pub hydrogen: f32,
}

pub fn player_movement(
    mut characters: Query<(&mut Transform, &mut Movable, &mut Vitals), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut movable, mut vitals) in &mut characters {
        if vitals.hydrogen >= 0.0 {
            if input.pressed(KeyCode::W) {
                movable.speed_y += movable.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::S) {
                movable.speed_y -= movable.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::D) {
                movable.speed_x += movable.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
            if input.pressed(KeyCode::A) {
                movable.speed_x -= movable.accel * time.delta_seconds();
                vitals.hydrogen -= 0.1;
            }
        }
        move_gizmo(&mut transform, &movable);
    }
}

pub fn vitals_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut characters: Query<(Entity,&mut Vitals), With<Player>>,
){
    for (player_entity, mut vitals) in &mut characters {
        vitals.vitals_timer.tick(time.delta());
        if vitals.vitals_timer.finished(){
            vitals.oxygen -= 0.5;
            if vitals.hydrogen < 100.0 {
                vitals.hydrogen += 0.5;
            } else {
                vitals.hydrogen = 100.0;
            }
            if vitals.oxygen <= 0.0 {
                vitals.health -= 1;
            }
        }
        if vitals.health <= 0 {
            info!("you died");
            commands.entity(player_entity).despawn();
        }
    }
}