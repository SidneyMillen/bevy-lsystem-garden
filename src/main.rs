use bevy::{
    asset::processor::ErasedProcessor,
    pbr::MaterialExtension,
    prelude::*,
    reflect::DynamicTypePath,
    render::{
        mesh::{shape::Quad, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        settings::{Backends, RenderCreation, WgpuSettings},
        view::VisibilitySystems,
        RenderPlugin,
    },
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_egui::{egui, systems::InputResources, EguiContext, EguiContexts, EguiPlugin, EguiSet};
use bevy_flycam::prelude::*;
use fractal_plant::{add_fractal_plant, FractalPlant};
use hilbert_curve::add_default_hilbert_curve;
use lsys_egui::{test_side_and_top_panel, PanelOccupiedScreenSpace};
use lsys_rendering::LineMaterial;
use lsystems::LSysDrawer;

use crate::lsys_rendering::RenderToLineList;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        .add_plugins(NoCameraPlayerPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugins(EguiPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::KeyE,
            move_descend: KeyCode::KeyQ,
            ..Default::default()
        })
        //.add_plugins(EguiPlugin)
        .init_resource::<PanelOccupiedScreenSpace>()
        .add_systems(
            Startup,
            (
                //add_default_hilbert_curve,
                add_fractal_plant,
                setup_camera,
            ),
        )
        .add_systems(
            PreUpdate,
            (
                test_side_and_top_panel,
                inspector_ui.after(test_side_and_top_panel),
            )
                .after(EguiSet::BeginFrame),
        )
        .add_systems(
            Update,
            (
                hilbert_curve::update_curve_materials,
                hilbert_curve::update_line_meshes.after(hilbert_curve::update_curve_materials),
                fractal_plant::update_plant_materials,
                fractal_plant::update_line_meshes.after(fractal_plant::update_plant_materials),
                rotate_all_drawers_towards_camera.after(fractal_plant::update_line_meshes),
                process_input_for_flycam,
            ),
        )
        .run();
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, 0.0, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);

    commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

    commands
        .spawn(Camera3dBundle {
            transform: camera_transform,
            ..default()
        })
        .insert(FlyCam);
}

fn reset_camera_position(
    mut commands: Commands,
    original_camera_transform: Res<OriginalCameraTransform>,
    mut camera_query: Query<&mut Transform, With<FlyCam>>,
) {
    let original_camera_transform = original_camera_transform.0.clone();

    for mut transform in &mut camera_query {
        *transform = original_camera_transform.clone();
    }
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

fn process_input_for_flycam(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Camera3d, &mut Transform), With<FlyCam>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (_, mut transform) in &mut query {
            transform.translation = Vec3::new(0.0, 0.0, 5.0);

            transform.look_at(CAMERA_TARGET, Vec3::Y);
        }
    }
}

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

fn rotate_all_drawers_towards_camera(
    camera_query: Query<&Transform, With<FlyCam>>,
    mut tree_query: Query<(&mut Transform, &FractalPlant), Without<FlyCam>>,
) {
    let camera_transform = camera_query.single();
    for (mut transform, _) in &mut tree_query {
        let facing = transform
            .clone()
            .looking_at(
                Vec3::new(
                    camera_transform.translation.x,
                    transform.translation.y,
                    camera_transform.translation.z,
                ),
                Vec3::Y,
            )
            .rotation;
        transform.rotation = facing;
    }
}
