use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use fractal_plant::add_fractal_plant;
use lsys_rendering::{FractalPlantUpdateEvent, LineMaterial};
use plant_pot::load_pot;

use serde::{Deserialize, Serialize};

mod camera;
mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;
mod pickup;
mod plant_pot;
mod save_load;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins((lsys_egui::MyEguiPlugin, camera::MyCameraPlugin))
        .add_systems(Startup, (add_fractal_plant, load_pot))
        .add_systems(Update, (fractal_plant::update_plant_meshes,))
        .add_event::<FractalPlantUpdateEvent>()
        .run();
}
