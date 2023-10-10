use bevy::prelude::*;

use crate::enums::game::GameState;
use super::despawn_screen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            //Change to on click switch
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup() {

}