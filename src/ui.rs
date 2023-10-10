use bevy::prelude::*;
use crate::gizmo::*;
use crate::player::*;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_vitals_ui);
    }
}

#[derive(Component)]
struct VitalsText;

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                node: Default::default(),
                style: Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(15.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new(
                            "Health",
                            TextStyle {
                                font_size: 16.0,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Oxygen",
                            TextStyle {
                                font_size: 16.0,
                                ..default()
                            },
                        ),
                        TextSection::new(
                            "Hydrogen",
                            TextStyle {
                                font_size: 16.0,
                                ..default()
                            },
                        ),
                    ]),
                    ..default()
                },
                VitalsText,
            ));
        });
}

fn update_vitals_ui (mut texts: Query<&mut Text, With<VitalsText>>, vitals: Query<&Vitals, With<Player>>) {
    for mut text in &mut texts {
        for vital in &vitals {
            text.sections[0].value = format!("Health: {:?} \n", vital.health);
            text.sections[1].value = format!("Oxygen: {:?}% \n", vital.oxygen);
            text.sections[2].value = format!("Hydrogen: {:?}% \n", vital.hydrogen);
        }
    }
}
