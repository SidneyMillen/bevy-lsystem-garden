use bevy::prelude::*;
use bevy::render::color::Color;
use std::f32::consts::PI;

use bevy::sprite::MaterialMesh2dBundle;

use crate::Position;

use crate::lsystems::LSysDrawer;

use crate::lsystems::LSysRules;

use crate::lsystems::LSys;

pub fn add_fractal_tree(mut commands: Commands) {
    commands
        .spawn(FractalTree {
            start_pos: Vec2::new(0.0, -200.0),
            start_angle: 0.0,
            line_length: 10.0,
            line_width: 2.0,
            branch_color: Color::rgb(1.0, 1.0, 0.0),
            leaf_color: Color::rgb(0.0, 1.0, 0.0),
            leaf_radius: 5.0,
            lsys: LSys {
                name: "fractal_tree".to_string(),
                rules: LSysRules::new(
                    vec!['0'],
                    vec![('1', "11".to_string()), ('0', "1[0]0".to_string())],
                ),
                iterations: 5,
            },
        })
        .insert(LSysDrawer {
            position: Position { x: 0.0, y: -200.0 },
            color: Color::rgb(1.0, 1.0, 1.0),
            angle: 0.0,
        });
}

pub(crate) fn draw_fractal_tree(
    query: Query<(&FractalTree, &LSysDrawer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (fractal_tree, drawer) in &query {
        let lsys = &fractal_tree.lsys;
        let evaluated_lsystem = lsys
            .rules
            .eval(&lsys.iterations)
            .expect("fractal tree lsystem evaluation failed");

        let evaluated_lsystem = evaluated_lsystem.chars();
        let start_pos = drawer.position.clone();
        let mut pos = start_pos.clone();
        let mut pos_stack: Vec<Position> = Vec::new();
        pos_stack.push(start_pos);
        let mut angle = drawer.angle.clone();
        let mut angle_stack: Vec<f32> = Vec::new();
        angle_stack.push(angle);
        let branch_color = fractal_tree.branch_color;
        let branch_length = fractal_tree.line_length;
        let branch_width = fractal_tree.line_width;
        let leaf_radius = fractal_tree.leaf_radius;
        let leaf_color = fractal_tree.leaf_color;

        for c in evaluated_lsystem {
            match c {
                '1' => {
                    let new_pos = Position {
                        x: pos.x + branch_length * -angle.sin(),
                        y: pos.y + branch_length * angle.cos(),
                    };
                    let mid_pos = Position {
                        x: (pos.x + new_pos.x) / 2.0,
                        y: (pos.y + new_pos.y) / 2.0,
                    };
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Rectangle::new(branch_width, branch_length))
                            .into(),
                        material: materials.add(fractal_tree.branch_color.clone()),
                        transform: Transform::from_translation(Vec3::new(
                            mid_pos.x, mid_pos.y, 0.0,
                        ))
                        .with_rotation(Quat::from_rotation_z(angle)),
                        ..Default::default()
                    });
                    pos = new_pos;
                }

                '0' => {
                    let new_pos = Position {
                        x: pos.x + branch_length * -angle.sin(),
                        y: pos.y + branch_length * angle.cos(),
                    };
                    let mid_pos = Position {
                        x: (pos.x + new_pos.x) / 2.0,
                        y: (pos.y + new_pos.y) / 2.0,
                    };
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Rectangle::new(branch_width, branch_length))
                            .into(),
                        material: materials.add(fractal_tree.branch_color.clone()),
                        transform: Transform::from_translation(Vec3::new(
                            mid_pos.x, mid_pos.y, 0.0,
                        ))
                        .with_rotation(Quat::from_rotation_z(angle)),
                        ..Default::default()
                    });
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::new(leaf_radius)).into(),
                        material: materials.add(leaf_color.clone()),
                        transform: Transform::from_translation(Vec3::new(
                            new_pos.x, new_pos.y, 0.0,
                        )),
                        ..Default::default()
                    });
                }
                '[' => {
                    pos_stack.push(pos.clone());
                    angle_stack.push(angle);
                    angle -= std::f32::consts::PI / 4.0;
                }
                ']' => {
                    pos = pos_stack.pop().unwrap();
                    angle = angle_stack.pop().unwrap();
                    angle += std::f32::consts::PI / 4.0;
                }
                _ => (),
            }
        }
    }
}

#[derive(Component)]
pub(crate) struct FractalTree {
    pub(crate) start_pos: Vec2,
    pub(crate) start_angle: f32,
    pub(crate) line_length: f32,
    pub(crate) line_width: f32,
    pub(crate) branch_color: Color,
    pub(crate) leaf_color: Color,
    pub(crate) leaf_radius: f32,
    pub(crate) lsys: LSys,
}
