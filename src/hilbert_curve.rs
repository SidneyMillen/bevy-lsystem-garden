use std::f32::consts::PI;

use crate::fractal_plant::LineList;
use crate::lsys_rendering::GenerateLineList;
use crate::lsys_rendering::LineMaterial;
use crate::lsystems::LSys;
use crate::lsystems::LSysDrawer;
use crate::lsystems::LSysRules;

use bevy::prelude::*;

#[derive(Component)]
pub struct HilbertCurve {
    pub(crate) start_pos: Vec3,
    pub(crate) start_heading: Quat,
    pub(crate) start_left: Quat,
    pub(crate) turn_angle: f32,
    pub(crate) segment_length: f32,
    pub(crate) color: Color,
    pub(crate) lsys: LSys,
    pub(crate) line_mesh: LineList,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<LineMaterial>,
}

impl HilbertCurve {
    pub fn new(
        start_pos: Vec3,
        start_heading: Quat,
        start_left: Quat,
        segment_length: f32,
        color: Color,
        lsys: LSys,
    ) -> Self {
        HilbertCurve {
            start_pos,
            start_heading,
            start_left,
            segment_length,
            color,
            turn_angle: PI / 2.0,
            lsys,
            line_mesh: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
            ..Default::default()
        }
    }
}

impl Default for HilbertCurve {
    fn default() -> Self {
        let mut tmp = Self {
            start_pos: Vec3::new(0.0, 0.0, 0.0),
            turn_angle: 90.0,
            start_heading: Quat::from_rotation_y(0.0),
            start_left: Quat::from_rotation_y(-PI / 2.0),
            segment_length: 0.1,
            color: Color::rgb(1.0, 1.0, 0.0),
            lsys: LSys {
                name: "hilbert_curve".to_string(),
                rules: LSysRules::new(
                    vec!['A'],
                    vec![
                        ('A', "B-F+CFC+F-D&F^D&F^D-F+&&CFC+F+B//".to_string()),
                        ('B', "A&F^CFB^F^D^^-F-D^F|F^B|FC^F^A//".to_string()),
                        ('C', "|D^|F^B-F+C^F^A&&FA&F^C+F+B^F^D//".to_string()),
                        ('D', "|CFB-F+B|FA&F^A&&FB-F+B|FC//".to_string()),
                    ],
                ),
                iterations: 2,
            },
            line_mesh: LineList { lines: vec![] },
            mesh_handle: Handle::<Mesh>::default(),
            material_handle: Handle::<LineMaterial>::default(),
        };
        tmp.line_mesh = tmp.generate_line_list();

        tmp
    }
}

pub fn add_default_hilbert_curve(
    mut commands: Commands,

    mut materials: ResMut<Assets<LineMaterial>>,
) {
    let curve = HilbertCurve::default();
    let curve_handle = curve.mesh_handle.clone();
    commands
        .spawn(curve)
        .insert(LSysDrawer { changed: true })
        .insert(MaterialMeshBundle {
            material: materials.add(LineMaterial::new(Color::rgb(1.0, 1.0, 1.0))),
            mesh: curve_handle,
            ..Default::default()
        });
}

impl GenerateLineList for HilbertCurve {
    fn generate_line_list(&self) -> LineList {
        let mut line_list = LineList { lines: vec![] };
        let mut current_pos = self.start_pos;
        let mut current_heading = self.start_heading;
        let mut current_left = current_heading * Quat::from_rotation_y(-PI / 2.0);
        let mut line_length = self.segment_length;

        for c in self.lsys.rules.eval(&self.lsys.iterations).unwrap().chars() {
            match c {
                'A' | 'B' | 'C' | 'D' | 'F' => {
                    let new_pos =
                        current_pos + current_heading.mul_vec3(Vec3::new(0.0, 0.0, line_length));
                    line_list.lines.push((current_pos, new_pos));
                    current_pos = new_pos;
                }

                '+' => {
                    current_heading = current_left.clone();
                    current_left = current_left.inverse();
                }

                '-' => {
                    current_heading = current_left.inverse();
                }
                '&' => current_heading = current_heading,
                '^' => {
                    current_heading = current_heading * Quat::from_rotation_x(-PI / 2.0);
                }
                '|' => {
                    current_heading = current_heading * Quat::from_rotation_y(PI);
                }
                '/' => {
                    current_heading = current_heading * Quat::from_rotation_z(PI / 2.0);
                }
                _ => {}
            }
        }

        line_list
    }
}

pub fn update_curve_materials(
    mut query: Query<(&mut HilbertCurve, &mut LSysDrawer)>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    for (mut curve, drawer) in &mut query {
        if curve.material_handle == Handle::<LineMaterial>::default() || drawer.changed {
            let old_handle = curve.material_handle.clone();
            curve.material_handle = dbg!(materials.add(LineMaterial::new(curve.color)).clone());
            materials.remove(&old_handle);
        }
    }
}

pub fn update_line_meshes(
    mut query: Query<(Entity, &mut HilbertCurve, &mut LSysDrawer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for (entity, mut curve, mut drawer) in &mut query {
        if drawer.changed {
            meshes.remove(&curve.mesh_handle);
            curve.line_mesh = curve.generate_line_list();
            let handle = meshes.add(curve.line_mesh.clone());
            curve.mesh_handle = handle.clone();

            commands
                .entity(entity)
                .insert(MaterialMeshBundle {
                    material: curve.material_handle.clone(),
                    mesh: handle,
                    ..Default::default()
                })
                .remove::<bevy::render::primitives::Aabb>();
            drawer.changed = false;
        }
    }
}
