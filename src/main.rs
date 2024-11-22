use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use bevy_panorbit_camera::PanOrbitCameraPlugin;
use cool_ball::CoolBallPlugin;
use editor::EditorPlugin;
use fractal_plant::{add_first_fractal_plant, add_new_fractal_plants};
use lsys_rendering::{FractalPlantUpdateEvent, LineMaterial};
use pickup::FocusedObject;
use plant_pot::load_pot;

use serde::{Deserialize, Serialize};

mod cool_ball;
mod editor;
mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;
mod pickup;
mod plant_pot;
mod player;
mod save_load;
mod stochastic_plants;
mod utils;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Default,
    Editor,
}

fn main() {
    App::new()
        .insert_state(GameState::Default)
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins((
            lsys_egui::MyEguiPlugin,
            player::MyPlayerPlugin,
            EditorPlugin,
            PanOrbitCameraPlugin, // pickup::PickupPlugin,
            CoolBallPlugin,
        ))
        .add_systems(Startup, (add_first_fractal_plant))
        .add_systems(
            Update,
            (
                add_new_fractal_plants,
                fractal_plant::update_plant_meshes,
                fractal_plant::update_plant_materials,
            ),
        )
        .add_event::<FractalPlantUpdateEvent>()
        .run();
}
