use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::render::mesh;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::VertexFormat;
use std::f32::consts::PI;

use bevy::sprite::MaterialMesh2dBundle;

use crate::lsys_rendering::LineMaterial;
use crate::lsys_rendering::RenderToLineList;
use crate::LineList;

use crate::lsystems::LSysDrawer;

use crate::lsystems::LSysRules;

use crate::lsystems::LSys;

pub fn add_fractal_tree(mut commands: Commands, mut materials: ResMut<Assets<LineMaterial>>) {
    let tree = FractalTree::default();
    let tree_handle = tree.mesh_handle.clone();
    commands
        .spawn(tree)
        .insert(LSysDrawer {
            transform: Transform::from_translation(Vec3::new(0.0, -200.0, 0.0)),
            color: Color::rgb(1.0, 1.0, 1.0),
            angle: 0.0,
            changed: true,
        })
        .insert(MaterialMeshBundle {
            material: materials.add(LineMaterial::new(Color::rgb(1.0, 1.0, 1.0))),
            mesh: tree_handle,
            ..Default::default()
        });
}

#[derive(Component)]
pub(crate) struct FractalTree {
    pub(crate) start_pos: Vec3,
    pub(crate) start_angle: f32,
    pub(crate) line_length: f32,
    pub(crate) branch_color: Color,
    pub(crate) lsys: LSys,
    pub(crate) line_mesh: LineList,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<LineMaterial>,
}

impl FractalTree {
    pub fn new(
        start_pos: Vec3,
        start_angle: f32,
        line_length: f32,
        branch_color: Color,
        lsys: LSys,
    ) -> Self {
        FractalTree {
            start_pos,
            start_angle,
            line_length,
            branch_color,
            lsys,
            line_mesh: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
        }
    }
}

impl Default for FractalTree {
    fn default() -> Self {
        let mut tmp = Self {
            start_pos: Vec3::new(0.0, 0.0, 0.0),
            start_angle: 0.0,
            line_length: 0.1,
            branch_color: Color::rgb(1.0, 1.0, 0.0),
            lsys: LSys {
                name: "fractal_tree".to_string(),
                rules: LSysRules::new(
                    vec!['0'],
                    vec![('1', "11".to_string()), ('0', "1[0]0".to_string())],
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
    mut query: Query<(Entity, &mut FractalTree, &mut LSysDrawer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (entity, mut tree, mut drawer) in &mut query {
        if drawer.changed {
            meshes.remove(&tree.mesh_handle);
            tree.line_mesh = tree.generate_line_mesh();
            let handle = meshes.add(tree.line_mesh.clone());
            tree.mesh_handle = handle.clone();

            commands.entity(entity).insert(MaterialMeshBundle {
                material: tree.material_handle.clone(),
                mesh: handle,
                ..Default::default()
            });
            drawer.changed = false;
        }
    }
}

pub fn update_tree_materials(
    mut query: Query<(&mut FractalTree, &mut LSysDrawer)>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    for (mut tree, mut drawer) in &mut query {
        if tree.material_handle == Handle::<LineMaterial>::default() || drawer.changed {
            let old_handle = tree.material_handle.clone();
            tree.material_handle =
                dbg!(materials.add(LineMaterial::new(tree.branch_color)).clone());
            materials.remove(&old_handle);
        }
    }
}

impl RenderToLineList for FractalTree {
    fn generate_line_mesh(&self) -> LineList {
        let mut line_list = LineList { lines: vec![] };
        let start_pos = self.start_pos;
        let mut v_pos = vec![start_pos];
        let iterations = self.lsys.iterations;

        let evaluated_lsystem = self.lsys.rules.eval(&iterations).unwrap();
        let mut pos = start_pos;
        let mut pos_stack: Vec<Vec3> = Vec::new();
        pos_stack.push(start_pos);
        let mut angle = self.start_angle;
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);
        let branch_length = self.line_length;
        let branch_color = self.branch_color;

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
                    angle -= PI / 4.0;
                }
                ']' => {
                    pos = pos_stack.pop().unwrap();
                    v_pos.push(pos);
                    angle = angle_stack.pop().unwrap() + PI / 4.0;
                }
                _ => {}
            }
        }

        dbg!(line_list)
    }
}
