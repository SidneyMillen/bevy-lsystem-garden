use std::ptr;

use bevy::{ecs::reflect::ReflectCommandExt, prelude::*, transform::commands};

use crate::player::{ActiveEntity, PlayerCam};

const PICKUP_POINT_OFFSET: f32 = 2.0;

#[derive(Component, Debug)]
pub struct ActiveEntityCandidate;

#[derive(Debug, Resource, Default, Clone)]
pub struct PlayerPickupPoint {
    pub distance: f32,
    pub global_pos: Vec3,
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerPickupPoint>()
            .add_systems(Update, move_active_object);
    }
}

fn setup_player_pickup_point(mut pickup_point: ResMut<PlayerPickupPoint>) {
    pickup_point.distance = PICKUP_POINT_OFFSET;
}

fn update_player_pickup_point(
    mut pickup_point: ResMut<PlayerPickupPoint>,
    player: Query<&Transform, With<PlayerCam>>,
) {
    let player_transform = player.get_single().unwrap();
    pickup_point.global_pos =
        player_transform.translation + player_transform.forward().normalize() * PICKUP_POINT_OFFSET;
}

pub fn move_active_object(
    pickup_point: Res<PlayerPickupPoint>,
    mut candidates: Query<(Entity, &mut Transform), With<ActiveEntityCandidate>>,
    active: Res<ActiveEntity>,
) {
    match active.id {
        Some(id) => {
            for (e, mut transform) in candidates.iter_mut() {
                if e.eq(&id) {
                    transform.translation = pickup_point.global_pos;
                }
            }
        }
        None => {}
    }
}
