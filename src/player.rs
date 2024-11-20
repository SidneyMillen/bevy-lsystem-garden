use bevy::{prelude::*, window::PrimaryWindow};
use bevy_flycam::{FlyCam, KeyBindings, MovementSettings, NoCameraPlayerPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::{
    fractal_plant::PlantSpawnPoint,
    lsys_egui::PanelOccupiedScreenSpace,
    pickup::{ActiveEntityCandidate, Holder},
    GameState,
};

const CAMERA_TARGET: Vec3 = Vec3::ZERO;
const MAX_FOCUS_DIST: f32 = 4.0;

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(Transform);

#[derive(Component, Debug)]
pub struct PlayerCam;

pub struct MyPlayerPlugin;

#[derive(Resource, Debug)]
pub struct ActiveEntity {
    pub id: Option<Entity>,
}

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NoCameraPlayerPlugin)
            .insert_resource(MovementSettings {
                sensitivity: 0.0008, // default: 0.00012
                speed: 6.0,          // default: 12.0
            })
            .insert_resource(KeyBindings {
                move_ascend: KeyCode::KeyE,
                move_descend: KeyCode::KeyQ,
                ..Default::default()
            })
            .insert_resource(ActiveEntity { id: None })
            .add_systems(OnEnter(GameState::Default), setup_camera)
            .add_systems(
                Update,
                (
                    update_camera_transform_system,
                    seek_active_object,
                    process_input_for_cam,
                )
                    .run_if(in_state(GameState::Default)),
            )
            .add_systems(
                PostUpdate,
                clamp_flycam_height.run_if(in_state(GameState::Default)),
            )
            .add_event::<CameraResetEvent>()
            .add_systems(OnExit(GameState::Default), exit_flycam);
    }
}

pub fn setup_camera(mut commands: Commands, q: Query<Entity, With<PlayerCam>>) {
    match q.get_single() {
        Ok(entity) => {
            commands.entity(entity).insert(FlyCam);
            dbg!("a");
        }
        Err(_) => {
            let camera_pos = Vec3::new(0.0, 0.0, 5.0);
            let camera_transform =
                Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);

            commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

            commands
                .spawn(Camera3dBundle {
                    transform: camera_transform,
                    ..default()
                })
                .insert(FlyCam)
                .insert(Holder::default())
                .insert(PlayerCam);
        }
    }
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

fn clamp_flycam_height(mut query: Query<&mut Transform, With<FlyCam>>) {
    let mut transform = query.get_single_mut().unwrap();

    transform.translation.y = transform.translation.y.clamp(0.0, 5.0);
}

pub fn process_input_for_cam(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut reset_events: EventWriter<CameraResetEvent>,
    player_transform: Query<&Transform, With<PlayerCam>>,
    mut commands: Commands,
) {
    let player_transform = player_transform.get_single().unwrap();
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        reset_events.send(CameraResetEvent);
    }
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        commands.spawn(PlantSpawnPoint(player_transform.translation));
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

    let distance_to_target = (CAMERA_TARGET - transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
}

fn seek_active_object(
    query: Query<(Entity, &Transform), With<ActiveEntityCandidate>>,
    player: Query<(&Transform), With<PlayerCam>>,
    mut active_entity: ResMut<ActiveEntity>,
) {
    let player_translation = player.get_single().unwrap().translation;

    let valid_candidate_distances = query
        .iter()
        .map(|t| (t.0, t.1.translation.distance(player_translation)))
        .filter(|t| t.1 < MAX_FOCUS_DIST);

    let best_candidate = valid_candidate_distances.min_by(|x, y| x.1.total_cmp(&y.1));
    active_entity.id = best_candidate.map_or(None, |x| Some(x.0));
}

fn exit_flycam(e: Query<Entity, With<FlyCam>>, mut commands: Commands) {
    commands.entity(e.get_single().unwrap()).remove::<FlyCam>();
}
