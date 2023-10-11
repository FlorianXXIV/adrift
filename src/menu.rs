use bevy::{app::AppExit, prelude::*};

use super::despawn_screen;
use crate::enums::game::GameState;
use crate::enums::settings::DisplayQuality;


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<OnSettingsMenuScreen>)
            .add_systems(
                Update,
                setting_button::<DisplayQuality>.run_if(in_state(MenuState::Settings))
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu))
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState{
    Main,
    Settings,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    for (interaction, menu_button_action) in &interaction_query {
        match menu_button_action {
            MenuButtonAction::Quit => app_exit_events.send(AppExit),
            MenuButtonAction::Play => {
                game_state.set(GameState::Game);
                menu_state.set(MenuState::Disabled);
            }
            MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
            MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
){
    for(interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
){
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>){
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(){

}

fn settings_menu_setup(){

}