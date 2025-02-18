use bevy::prelude::*;
use systems::{add_people, greet_people, hello_world, update_people};
mod components;
mod systems;

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins) - removing default plugins only while learning ECS basics
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}
