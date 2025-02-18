use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
