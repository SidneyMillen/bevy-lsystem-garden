use bevy::{
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
};

use fractal_plant::{add_first_fractal_plant, add_new_fractal_plants};
use lsys_rendering::{FractalPlantUpdateEvent, LineMaterial};
use pickup::FocusedObject;
use plant_pot::load_pot;

use serde::{Deserialize, Serialize};

mod fractal_plant;
mod hilbert_curve;
mod lsys_egui;
mod lsys_rendering;
mod lsystems;
mod pickup;
mod plant_pot;
mod player;
mod save_load;
mod states;

#[derive(Resource)]
enum GameState{
    Default,
    Editor(FocusedObject)
}

fn setup(mut commands: Commands){
    commands.insert_resource(GameState::Default);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<LineMaterial>::default()))
        //.add_plugins(NoCameraPlayerPlugin)
        .add_plugins((
            lsys_egui::MyEguiPlugin,
            player::MyPlayerPlugin,
            // pickup::PickupPlugin,
        ))
        .add_systems(Startup, (add_first_fractal_plant, setup))
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
