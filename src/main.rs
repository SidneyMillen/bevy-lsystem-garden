use std::{borrow::Cow, f32::consts::PI};

use bevy::{
    prelude::*,
    render::{
        mesh::shape::Quad,
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use fractal_tree::FractalTree;
use lsys_egui::fractal_tree_ui;
use lsystems::LSysDrawer;

mod fractal_tree;
mod lsys_egui;
mod lsystems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                device_label: Some(Cow::Borrowed("WGPU Device")),
                ..Default::default()
            }),
            synchronous_pipeline_compilation: false,
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup_camera, fractal_tree::add_fractal_tree))
        .add_systems(
            Update,
            (move_drawn_tree_system, fractal_tree::draw_fractal_tree),
        )
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

fn move_drawn_tree_system(mut query: Query<(&mut LSysDrawer)>, time: Res<Time>) {
    for mut drawer in &mut query {
        drawer.position.x += time.delta_seconds() * 10.0;
        drawer.position.y += time.delta_seconds() * 10.0;
    }
}
