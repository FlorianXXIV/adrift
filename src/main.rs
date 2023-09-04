use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::player::Player;

mod enums;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Adrift".into(),
                        resolution: (640.0, 480.0).into(),

                        ..default()
                    }),
                    ..default()
                })
            .build()
        )
        .add_systems(Startup, setup)
        .add_systems(Update, player::character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1024.0,
        min_height: 576.0,
    };

    commands.spawn(camera);
    info!("Camera Loaded");

    let texture = asset_server.load("sprites/placeholder.png");

    commands.spawn((SpriteBundle {
        texture,
        ..default()
    },
    Player{ accel: 10.0, speed_x: 0.0, speed_y: 0.0 },
    ));
    info!("test image loaded");
}