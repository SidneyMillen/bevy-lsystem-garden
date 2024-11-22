use bevy::ecs::reflect::ReflectCommandExt;
use bevy::prelude::*;
use bevy::render::color::Color;

use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use bevy_egui::egui::Color32;
use std::f32::consts::PI;

use bevy::sprite::MaterialMesh2dBundle;

use crate::lsys_egui::SideMenuOptions;
use crate::lsys_rendering::{FractalPlantUpdateEvent, LineMaterial};
use crate::lsys_rendering::{GenerateLineList, LineMesh};
use crate::pickup::ActiveEntityCandidate;
use crate::utils::{Counter, Level};
use crate::{save_load, GameState};

use crate::lsystems::LSysDrawer;

use crate::lsystems::LSysRules;

use crate::lsystems::LSys;

pub fn add_first_fractal_plant(
    mut commands: Commands,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut update_writer: EventWriter<FractalPlantUpdateEvent>,
    assets: Res<AssetServer>,
) {
    spawn_plant_at(
        &assets,
        &mut commands,
        &mut materials,
        &mut update_writer,
        Vec3::splat(0.0),
    );
}

fn spawn_plant_at(
    assets: &Res<'_, AssetServer>,
    mut commands: &mut Commands<'_, '_>,
    mut materials: &mut ResMut<'_, Assets<LineMaterial>>,
    mut update_writer: &mut EventWriter<'_, FractalPlantUpdateEvent>,
    pos: &Vec3,
) {
    let pot: Handle<Scene> = assets.load("pot.glb#Scene0");

    let tree = FractalPlant::default();
    let plant_mesh = LineMesh::default();
    let plant_mesh_handle = plant_mesh.mesh_handle.clone();
    let id = commands
        .spawn((tree, plant_mesh))
        .insert(LSysDrawer { changed: true })
        .insert(ActiveEntityCandidate)
        .insert(MaterialMeshBundle {
            material: materials.add(LineMaterial::new(Color::rgb(1.0, 1.0, 1.0))),
            mesh: plant_mesh_handle,
            transform: Transform::from_translation(*pos),
            ..Default::default()
        })
        .insert(SceneBundle {
            scene: pot,
            ..default()
        })
        .insert((Counter(0), Level(1)))
        .id();
    update_writer.send(FractalPlantUpdateEvent::MESH(id));
    update_writer.send(FractalPlantUpdateEvent::MATERIAL(id));
}
pub fn add_new_fractal_plants(
    mut commands: Commands,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut update_writer: EventWriter<FractalPlantUpdateEvent>,
    spawn_q: Query<(Entity, &PlantSpawnPoint)>,
    assets: Res<AssetServer>,
) {
    for (entity, PlantSpawnPoint(pos)) in spawn_q.iter() {
        spawn_plant_at(
            &assets,
            &mut commands,
            &mut materials,
            &mut update_writer,
            pos,
        );
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct PlantSpawnPoint(pub Vec3);

#[derive(Component, Serialize, Deserialize, Debug)]
pub(crate) struct FractalPlant {
    pub(crate) start_pos: Vec3,
    pub(crate) start_angle: f32,
    pub(crate) turn_angle: f32,
    pub(crate) line_length: f32,
    pub(crate) branch_color: Color,
    pub(crate) lsys: LSys,
    #[serde(skip_serializing, skip_deserializing)]
    pub mesh_handle: Handle<Mesh>,
    #[serde(skip_serializing, skip_deserializing)]
    pub material_handle: Handle<LineMaterial>,
}

impl FractalPlant {
    pub fn new(
        start_pos: Vec3,
        start_angle: f32,
        line_length: f32,
        branch_color: Color,
        lsys: LSys,
    ) -> Self {
        FractalPlant {
            start_pos,
            start_angle,
            line_length,
            branch_color,
            lsys,
            ..Default::default()
        }
    }
}

impl Default for FractalPlant {
    fn default() -> Self {
        let plant = Self {
            start_pos: Vec3::new(0.0, 0.0, 0.0),
            turn_angle: PI / 4.0,
            start_angle: 0.0,
            line_length: 0.1,
            branch_color: Color::rgb(1.0, 1.0, 0.0),
            lsys: LSys {
                name: "fractal_tree".to_string(),
                rules: LSysRules::new(
                    vec!['0'],
                    vec![('1', "11".to_string()), ('0', "1[-0]+0".to_string())],
                ),
            },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
        };

        plant
    }
}

pub fn update_plant_meshes(
    mut query: Query<(Entity, &mut FractalPlant, &Level, &Counter), Changed<FractalPlant>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (entity, mut plant, level, counter) in query.iter_mut() {
        let mut new_line_list = Vec::<(Vec3, Vec3)>::new();
        let start_pos = plant.start_pos;
        let mut v_pos = vec![start_pos];
        let iterations: usize = counter.0;

        let mut pos = start_pos;
        let mut pos_stack: Vec<Vec3> = Vec::new();
        pos_stack.push(start_pos);
        let mut heading: Quat = Quat::from_rotation_z(plant.start_angle);
        let mut angle_stack: Vec<Quat> = Vec::new();
        angle_stack.push(heading);
        let branch_length = plant.line_length;

        let evaluated_lsystem = plant.lsys.rules.eval(&iterations).unwrap_or("".to_string());

        for c in evaluated_lsystem.chars() {
            match c {
                '1' => {
                    let new_pos = heading.mul_vec3(Vec3::new(0.0, branch_length, 0.0)) + pos;
                    new_line_list.push((pos, new_pos));

                    pos = new_pos;
                }
                '0' => {
                    let new_pos = heading.mul_vec3(Vec3::new(0.0, branch_length, 0.0)) + pos;
                    new_line_list.push((pos, new_pos));
                }
                '[' => {
                    pos_stack.push(pos);
                    angle_stack.push(heading);
                }
                '-' => {
                    heading *= Quat::from_rotation_z(-plant.turn_angle);
                }
                '+' => {
                    heading *= Quat::from_rotation_z(plant.turn_angle);
                }
                '<' => {
                    heading *= Quat::from_rotation_x(-plant.turn_angle);
                }
                '>' => {
                    heading *= Quat::from_rotation_x(plant.turn_angle);
                }
                ']' => {
                    pos = pos_stack.pop().unwrap_or(pos);
                    v_pos.push(pos);
                    heading = angle_stack.pop().unwrap_or(heading);
                }
                _ => {}
            }
        }

        let handle = meshes.add(LineList {
            lines: new_line_list,
        });
        plant.mesh_handle = handle.clone();
        commands
            .entity(entity)
            .remove::<Handle<Mesh>>()
            .remove::<bevy::render::primitives::Aabb>()
            .insert(handle.clone());
    }
}

pub fn update_plant_materials(
    mut query: Query<(Entity, &mut FractalPlant), Changed<FractalPlant>>,
    mut mats: ResMut<Assets<LineMaterial>>,
    mut material_updates: EventReader<FractalPlantUpdateEvent>,
    mut commands: Commands,
) {
    for (entity, mut plant) in query.iter_mut() {
        let new_material = LineMaterial::new(plant.branch_color);
        plant.material_handle = mats.add(new_material);
        commands
            .entity(entity)
            .remove::<Handle<LineMaterial>>()
            .insert(plant.material_handle.clone());
    }
    material_updates.clear();
}

impl SideMenuOptions for FractalPlant {
    fn side_menu_options(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        active_id: Entity,
        commands: &mut Commands,
        gs: &mut NextState<GameState>,
    ) {
        let mut mat_changed = false;
        let old_length = self.line_length;

        let mut new_start_angle_deg = self.start_angle.to_degrees();
        ui.label("Fractal Tree Options");
        ui.horizontal(|ui| {
            ui.label("Start Angle");
            ui.add(bevy_egui::egui::Slider::new(
                &mut new_start_angle_deg,
                0.0..=360.0,
            ));
            if new_start_angle_deg != self.start_angle.to_degrees() {
                self.start_angle = new_start_angle_deg.to_radians();
            }
        });
        let mut new_turn_angle_deg = self.turn_angle.to_degrees();
        ui.horizontal(|ui| {
            ui.label("Turn Angle");
            ui.add(bevy_egui::egui::Slider::new(
                &mut new_turn_angle_deg,
                0.0..=180.0,
            ));
            if new_turn_angle_deg != self.turn_angle.to_degrees() {
                self.turn_angle = new_turn_angle_deg.to_radians();
            }
        });
        ui.horizontal(|ui| {
            ui.label("Line Length");
            ui.add(bevy_egui::egui::Slider::new(
                &mut self.line_length,
                0.0..=0.3,
            ));
        });
        let mut col = Color32::from_rgb(
            (self.branch_color.r() * 255.0) as u8,
            (self.branch_color.g() * 255.0) as u8,
            (self.branch_color.b() * 255.0) as u8,
        );
        let old_col = col.clone();
        ui.horizontal(|ui| {
            ui.label("Branch Color");
            ui.color_edit_button_srgba(&mut col);
        });
        if col != old_col {
            self.branch_color = Color::rgb(
                col.r() as f32 / 255.0,
                col.g() as f32 / 255.0,
                col.b() as f32 / 255.0,
            );
            mat_changed = true;
        }

        ui.label("Rules:");
        for (i, (k, v)) in self.lsys.rules.rules.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label("Rule:");
                ui.label(k.to_string());
                ui.label(" -> ");
                let old_v = v.clone();
                ui.text_edit_singleline(v);
                if old_v != *v {}
            });
        }
        ui.label("Axiom:");
        let mut new_axiom = self.lsys.rules.axiom.clone().iter().collect::<String>();
        ui.text_edit_singleline(&mut new_axiom);
        if new_axiom != self.lsys.rules.axiom.iter().collect::<String>() {
            self.lsys.rules.axiom = new_axiom.chars().collect();
        }

        ui.label("Name");
        let mut new_name = self.lsys.name.clone();
        ui.text_edit_singleline(&mut new_name);
        if new_name != self.lsys.name {
            self.lsys.name = new_name;
        }
        if ui.button("Save configuration").clicked() {
            let _ = save_load::serialize_to_file(&self, &self.lsys.name.clone());
        }
        if ui.button("Load configuration").clicked() {
            let loaded: FractalPlant = save_load::deserialize_from_file(&self.lsys.name.clone())
                .unwrap_or(FractalPlant::default());

            self.start_pos = loaded.start_pos;
            self.start_angle = loaded.start_angle;
            self.turn_angle = loaded.turn_angle;
            self.line_length = loaded.line_length;
            self.branch_color = loaded.branch_color;
            self.lsys = loaded.lsys;
            mat_changed = true;
        }
    }
}
/// A list of lines with a start and end position
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineList {
    pub(crate) lines: Vec<(Vec3, Vec3)>,
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
