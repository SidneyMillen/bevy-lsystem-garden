use bevy::prelude::*;

use crate::camera::PlayerCam;

const MAX_PICKUP_DIST: f32 = 2.0;

#[derive(Component, Debug)]
pub struct Pickup;

#[derive(Component)]
pub struct PlayerPickupPoint {
    pub distance: f32,
}

pub fn setup_player_pickup_point(mut commands: Commands, query: Query<Entity, With<PlayerCam>>) {
    let player_entity = query.get_single().unwrap();
    let player_pickup = PlayerPickupPoint { distance: 2.0 };
    commands.entity(player_entity).insert(player_pickup);
}

fn find_closest_pickup(
    pickups: Vec<Vec3>,
    pickup_point: &PlayerPickupPoint,
    player_transform: &Transform,
) -> Option<Pickup> {
    let global_player_pickup_point: Vec3 = player_transform.translation
        + (pickup_point.distance * player_transform.forward().normalize());
    let mut pickup_distances: Vec<(f32, usize)> = pickups
        .iter()
        .map(|translation| (translation.clone() - global_player_pickup_point).length())
        .collect();
}
