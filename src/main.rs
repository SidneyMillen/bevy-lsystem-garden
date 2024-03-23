use bevy::{
    prelude::*,
    render::{
        mesh::{shape::Quad, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use fractal_tree::{add_fractal_tree, FractalTree};
use lsys_egui::{fractal_tree_ui, test_side_and_top_panel, PanelOccupiedScreenSpace};
use lsys_rendering::LineMaterial;
use lsystems::LSysDrawer;

use crate::lsys_rendering::RenderToLineList;

mod fractal_tree;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        .add_plugins(EguiPlugin)
        .init_resource::<PanelOccupiedScreenSpace>()
        .add_systems(Startup, (setup_camera, add_fractal_tree))
        .add_systems(Update, test_side_and_top_panel)
        .add_systems(Update, update_camera_transform_system)
        .add_systems(
            Update,
            (
                fractal_tree::update_line_meshes,
                fractal_tree::update_tree_materials,
            ),
        )
        .run();
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, 0.0, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);

    commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..default()
    });
}

fn update_camera_transform_system(
    occupied_screen_space: Res<PanelOccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}

#[derive(Component, Clone, Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn move_drawn_tree_system(mut query: Query<(&mut LSysDrawer)>, time: Res<Time>) {
    for mut drawer in &mut query {
        drawer.transform.translation.x += time.delta_seconds() * 10.0;
        drawer.transform.translation.y += time.delta_seconds() * 10.0;
    }
}

struct tree_marker {}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}
