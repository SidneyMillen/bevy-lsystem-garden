use bevy::{prelude::*, window::PrimaryWindow};
use bevy_flycam::FlyCam;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::lsys_egui::PanelOccupiedScreenSpace;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(Transform);

#[derive(Component, Debug)]
pub struct PlayerCam;
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
        })
        .insert(PlayerCam);
}

#[derive(Debug, Event)]
pub struct CameraResetEvent;

pub fn reset_camera_position(
    original_camera_transform: Res<OriginalCameraTransform>,
    mut camera_query: Query<&mut Transform, With<FlyCam>>,
    mut reset_events: EventReader<CameraResetEvent>,
) {
    for _ in reset_events.read().into_iter() {
        for mut transform in &mut camera_query {
            transform.translation = original_camera_transform.translation;
            transform.look_at(CAMERA_TARGET, Vec3::Y);
        }
    }
}

pub fn process_input_for_cam(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut reset_events: EventWriter<CameraResetEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        reset_events.send(CameraResetEvent);
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
