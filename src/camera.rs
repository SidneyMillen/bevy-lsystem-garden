use bevy::{prelude::*, window::PrimaryWindow};
use bevy_flycam::FlyCam;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::lsys_egui::PanelOccupiedScreenSpace;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);
pub fn setup_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, 0.0, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);

    commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

    commands
        .spawn(Camera3dBundle {
            transform: camera_transform,
            ..default()
        })
        .insert(PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Right,
            modifier_pan: Some(KeyCode::ShiftLeft),
            ..Default::default()
        });
}

pub fn reset_camera_position(
    mut commands: Commands,
    original_camera_transform: Res<OriginalCameraTransform>,
    mut camera_query: Query<&mut Transform, With<FlyCam>>,
) {
    let original_camera_transform = original_camera_transform.0.clone();

    for mut transform in &mut camera_query {
        *transform = original_camera_transform.clone();
    }
}

pub fn process_input_for_cam(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Camera3d, &mut Transform), With<PanOrbitCamera>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for (_, mut transform) in &mut query {
            transform.translation = Vec3::new(0.0, 0.0, 5.0);
            transform.look_at(CAMERA_TARGET, Vec3::Y);
        }
    }
}

fn update_camera_transform_system(
    occupied_screen_space: Res<PanelOccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}
