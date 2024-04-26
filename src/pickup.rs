use std::ptr;

use bevy::{ecs::reflect::ReflectCommandExt, prelude::*, transform::commands};

use crate::{
    lsys_egui::SideMenuOptions,
    player::{ActiveEntity, PlayerCam},
};

const PICKUP_POINT_OFFSET: f32 = 2.0;

#[derive(Component, Debug)]
pub struct ActiveEntityCandidate;

#[derive(Component, Debug)]
pub struct Holder {
    held: bool,
}

pub enum HolderEvent {
    PICKUP,
    DROP,
}
#[derive(Event)]
pub struct PickupDropEvent(HolderEvent);

impl Holder {
    fn hold(&mut self) {
        self.held = true;
    }
    fn drop(&mut self) {
        self.held = false;
    }
    fn held(&self) -> bool {
        self.held
    }
}
impl Default for Holder {
    fn default() -> Self {
        Holder { held: false }
    }
}

impl SideMenuOptions<PickupDropEvent> for Holder {
    fn side_menu_options(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        event_writer: &mut EventWriter<PickupDropEvent>,
        entity_id: Entity,
        active_id: Entity,
    ) {
        match entity_id == active_id {
            true => {
                ui.label("Pickup/Drop");
                if ui.button("Pickup").clicked() {
                    event_writer.send(PickupDropEvent(HolderEvent::PICKUP));
                }
                if ui.button("Drop").clicked() {
                    event_writer.send(PickupDropEvent(HolderEvent::DROP));
                }
            }
            false => {}
        }
    }
}

#[derive(Debug, Resource, Default, Clone)]
pub struct PlayerPickupPoint {
    pub distance: f32,
    pub global_pos: Vec3,
}

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerPickupPoint>()
            .add_systems(Startup, setup_player_pickup_point)
            .add_systems(
                Update,
                (update_player_pickup_point, move_active_entity_to_hold).chain(),
            );
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

pub fn move_active_entity_to_hold(
    pickup_point: Res<PlayerPickupPoint>,
    mut candidates: Query<(Entity, &mut Transform), With<ActiveEntityCandidate>>,
    active: Res<ActiveEntity>,
    holder: Query<&Holder>,
) {
    let holder = holder.get_single().unwrap();
    if holder.held() {
        match active.id {
            Some(id) => {
                for (e, mut transform) in candidates.iter_mut() {
                    if e.eq(&id) {
                        dbg!("pickup match: {}", e);
                        transform.translation = pickup_point.global_pos;
                        dbg!("pickup point: {}", pickup_point.global_pos);
                    }
                }
            }
            None => {}
        }
    }
}
