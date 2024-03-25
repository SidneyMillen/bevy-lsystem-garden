use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::VertexFormat;
use bevy_egui::egui::Color32;
use std::f32::consts::PI;

use bevy::sprite::MaterialMesh2dBundle;

use crate::lsys_egui::SideMenuOptions;
use crate::lsys_rendering::LineMaterial;
use crate::lsys_rendering::RenderToLineList;
use crate::LineList;

use crate::lsystems::LSysDrawer;

use crate::lsystems::LSysRules;

use crate::lsystems::LSys;

pub fn add_fractal_tree(mut commands: Commands, mut materials: ResMut<Assets<LineMaterial>>) {
    let tree = FractalPlant::default();
    let tree_handle = tree.mesh_handle.clone();
    commands
        .spawn(tree)
        .insert(LSysDrawer { changed: true })
        .insert(MaterialMeshBundle {
            material: materials.add(LineMaterial::new(Color::rgb(1.0, 1.0, 1.0))),
            mesh: tree_handle,
            ..Default::default()
        });
}

#[derive(Component)]
pub(crate) struct FractalPlant {
    pub(crate) start_pos: Vec3,
    pub(crate) start_angle: f32,
    pub(crate) turn_angle: f32,
    pub(crate) line_length: f32,
    pub(crate) branch_color: Color,
    pub(crate) lsys: LSys,
    pub(crate) line_mesh: LineList,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<LineMaterial>,
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
            line_mesh: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
            ..Default::default()
        }
    }
}

impl Default for FractalPlant {
    fn default() -> Self {
        let mut tmp = Self {
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
                iterations: 2,
            },
            line_mesh: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
        };
        tmp.line_mesh = tmp.generate_line_mesh();

        tmp
    }
}

pub fn update_line_meshes(
    mut query: Query<(Entity, &mut FractalPlant, &mut LSysDrawer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (entity, mut tree, mut drawer) in &mut query {
        if drawer.changed {
            meshes.remove(&tree.mesh_handle);
            tree.line_mesh = tree.generate_line_mesh();
            let handle = meshes.add(tree.line_mesh.clone());
            tree.mesh_handle = handle.clone();

            commands
                .entity(entity)
                .insert(MaterialMeshBundle {
                    material: tree.material_handle.clone(),
                    mesh: handle,
                    ..Default::default()
                })
                .remove::<bevy::render::primitives::Aabb>();
            drawer.changed = false;
        }
    }
}

pub fn update_tree_materials(
    mut query: Query<(&mut FractalPlant, &mut LSysDrawer)>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    for (mut tree, drawer) in &mut query {
        if tree.material_handle == Handle::<LineMaterial>::default() || drawer.changed {
            let old_handle = tree.material_handle.clone();
            tree.material_handle =
                dbg!(materials.add(LineMaterial::new(tree.branch_color)).clone());
            materials.remove(&old_handle);
        }
    }
}

impl RenderToLineList for FractalPlant {
    fn generate_line_mesh(&self) -> LineList {
        let mut line_list = LineList { lines: vec![] };
        let start_pos = self.start_pos;
        let mut v_pos = vec![start_pos];
        let iterations = self.lsys.iterations;

        let mut pos = start_pos;
        let mut pos_stack: Vec<Vec3> = Vec::new();
        pos_stack.push(start_pos);
        let mut angle = self.start_angle;
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);
        let branch_length = self.line_length;

        let evaluated_lsystem = self.lsys.rules.eval(&iterations).unwrap_or("".to_string());

        for c in evaluated_lsystem.chars() {
            match c {
                '1' => {
                    let new_pos = Vec3::new(
                        pos.x + branch_length * -angle.sin(),
                        pos.y + branch_length * angle.cos(),
                        0.0,
                    );
                    line_list.lines.push((pos, new_pos));

                    pos = new_pos;
                }
                '0' => {
                    let new_pos = Vec3::new(
                        pos.x + branch_length * -angle.sin(),
                        pos.y + branch_length * angle.cos(),
                        0.0,
                    );
                    line_list.lines.push((pos, new_pos));
                }
                '[' => {
                    pos_stack.push(pos);
                    angle_stack.push(angle);
                }
                '-' => {
                    angle -= self.turn_angle;
                }
                '+' => {
                    angle += self.turn_angle;
                }
                ']' => {
                    pos = pos_stack.pop().unwrap_or(pos);
                    v_pos.push(pos);
                    angle = angle_stack.pop().unwrap_or(angle);
                }
                _ => {}
            }
        }

        line_list
    }
}

impl SideMenuOptions for FractalPlant {
    fn side_menu_options(&mut self, drawer: &mut LSysDrawer, ui: &mut bevy_egui::egui::Ui) {
        let old_length = self.line_length;
        let old_iterations = self.lsys.iterations;

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
                drawer.changed = true;
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
                drawer.changed = true;
            }
        });
        ui.horizontal(|ui| {
            ui.label("Line Length");
            ui.add(bevy_egui::egui::Slider::new(
                &mut self.line_length,
                0.0..=1.0,
            ));
        });
        ui.horizontal(|ui| {
            ui.label("Iterations");
            ui.add(bevy_egui::egui::Slider::new(
                &mut self.lsys.iterations,
                0..=8,
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
        if dbg!(col != old_col) {
            self.branch_color = Color::rgb(
                col.r() as f32 / 255.0,
                col.g() as f32 / 255.0,
                col.b() as f32 / 255.0,
            );
            drawer.changed = true;
        }

        ui.label("Rules:");
        for (i, (k, v)) in self.lsys.rules.rules.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label("Rule:");
                ui.label(k.to_string());
                ui.label(" -> ");
                let old_v = v.clone();
                ui.text_edit_singleline(v);
                if old_v != *v {
                    drawer.changed = true;
                }
            });
        }
        ui.label("Axiom:");
        let mut new_axiom = self.lsys.rules.axiom.clone().iter().collect::<String>();
        ui.text_edit_singleline(&mut new_axiom);
        if new_axiom != self.lsys.rules.axiom.iter().collect::<String>() {
            self.lsys.rules.axiom = new_axiom.chars().collect();
            drawer.changed = true;
        }

        if old_length != self.line_length || old_iterations != self.lsys.iterations {
            drawer.changed = true;
        }
    }
}