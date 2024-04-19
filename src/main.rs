use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use fractal_plant::add_fractal_plant;
use lsys_rendering::LineMaterial;
use plant_pot::load_pot;

use serde::{Deserialize, Serialize};

mod camera;
mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;
mod plant_pot;
mod save_load;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins((lsys_egui::MyEguiPlugin, camera::MyCameraPlugin))
        .add_systems(Startup, (add_fractal_plant, load_pot))
        .add_systems(
            Update,
            (
                fractal_plant::update_plant_materials,
                fractal_plant::update_line_meshes.after(fractal_plant::update_plant_materials),
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
