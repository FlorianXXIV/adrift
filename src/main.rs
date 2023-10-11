mod enums;
mod ui;
mod player;
mod gizmo;
mod splash;
mod menu;
mod game;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::GameUI;
use crate::enums::game::GameState;
use crate::game::GamePlugin;
use crate::gizmo::*;
use crate::player::*;


fn main() {
    App::new()
        .add_state::<GameState>()
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
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands){
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1920.0,
        min_height: 1080.0,
    };

    commands.spawn(camera);
    info!("Camera Loaded");
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands){
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}