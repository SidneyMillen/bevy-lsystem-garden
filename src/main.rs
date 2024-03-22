use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::shape::Quad, sprite::MaterialMesh2dBundle};

mod fractal_tree;
mod lsystems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, fractal_tree::add_fractal_tree))
        .add_systems(Update, fractal_tree::draw_fractal_tree)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component, Clone, Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
