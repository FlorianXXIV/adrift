use std::fmt::Pointer;
use bevy::prelude::*;

use crate::enums::game::GameState;
use super::despawn_screen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            //Change to on click switch
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("assets/sprites/test.png");

    commands
        .spawn((
            NodeBundle{
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    //insert the timer as a resource
    commands.insert_resource(SplashTimer(
        Timer::from_seconds(1.0,TimerMode::Once)
    ));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}