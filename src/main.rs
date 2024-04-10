use bevy::{
    prelude::*,
    render::{
        mesh::{shape::Quad, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        settings::{Backends, RenderCreation, WgpuSettings},
        view::VisibilitySystems,
        RenderPlugin,
    },
};

use bevy_egui::{egui, systems::InputResources, EguiContext, EguiContexts, EguiPlugin, EguiSet};
use bevy_flycam::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use camera::{process_input_for_cam, setup_camera};
use fractal_plant::{add_fractal_plant, FractalPlant};
use lsys_egui::{inspector_ui, test_side_and_top_panel, PanelOccupiedScreenSpace};
use lsys_rendering::LineMaterial;
use std::path::Path;

use save_load::serialize_to_file;

use serde::{Deserialize, Serialize};

mod camera;
mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;
mod save_load;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::KeyE,
            move_descend: KeyCode::KeyQ,
            ..Default::default()
        })
        .init_resource::<PanelOccupiedScreenSpace>()
        .add_systems(Startup, (add_fractal_plant, setup_camera))
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
                process_input_for_cam,
                hilbert_curve::update_curve_materials,
                hilbert_curve::update_line_meshes.after(hilbert_curve::update_curve_materials),
                fractal_plant::update_plant_materials,
                fractal_plant::update_line_meshes.after(fractal_plant::update_plant_materials),
                test_serialize,
            ),
        )
        .run();
}

/// A list of lines with a start and end position
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn test_serialize(mut query: Query<(&FractalPlant)>) {
    for tree in query.iter() {
        serialize_to_file(tree)
    }
}
