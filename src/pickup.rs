use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Pickup;

pub struct PlayerPickupPoint;

pub fn setup_player_pickup_point(query: Query<&Transform, With<PlayerCam>>) {
    pickup_point = PlayerPickupPoint;
}
