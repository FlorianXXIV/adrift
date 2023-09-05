use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gizmos::player::*;
use gizmos::gizmo::*;
use ui::GameUI;

mod enums;
mod gizmos;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Adrift".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
            .build()
        )
        .add_plugins(
            WorldInspectorPlugin::default()
                .run_if(input_toggle_active(false, KeyCode::Escape))
        )
        .add_plugins((PlayerPlugin, GameUI))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1280.0,
        min_height: 720.0,
    };

    commands.spawn(camera);
    info!("Camera Loaded");

    let texture = asset_server.load("sprites/placeholder.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Movable {
            accel: 10.0,
            speed_x: 0.0,
            speed_y: 0.0
        },
        Vitals {
            vitals_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            health: 100,
            oxygen: 100.0,
            hydrogen: 100.0,
        },
        Name::new("Player"),
        Player,
    ));
    info!("Spawned Player");
}