use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, (hello_world, greet_people));
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Dude Manbro".to_string())));
    commands.spawn((Person, Name("Julia Yeet".to_string())));
    commands.spawn((Person, Name("Example Lastname".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn hello_world(){
    println!("Hello World!")
}