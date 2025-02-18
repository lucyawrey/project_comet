use crate::components::Name;
use crate::components::Person;
use bevy::prelude::*;

pub fn hello_world() {
    println!("hello world!");
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Stefanie".to_string())));
    commands.spawn((Person, Name("Laura".to_string())));
    commands.spawn((Person, Name("Lucy".to_string())));
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Stefanie" {
            name.0 = "Stefieany".to_string();
            break; // We don't need to change any other names.
        }
    }
}
