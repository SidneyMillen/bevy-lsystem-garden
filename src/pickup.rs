use std::ptr;

use bevy::{ecs::reflect::ReflectCommandExt, prelude::*, transform::commands};

use crate::{
    lsys_egui::SideMenuOptions,
    player::{ActiveEntity, PlayerCam},
};

const PICKUP_POINT_OFFSET: f32 = 2.0;

#[derive(Component, Debug)]
pub struct ActiveEntityCandidate;

impl SideMenuOptions for ActiveEntityCandidate {
    fn side_menu_options(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        id: Entity,
        commands: &mut Commands,
    ) {
        if ui.button("pick up").clicked() {
            commands.entity(id).insert(HeldObject);
        }
        if ui.button("drop").clicked() {
            drop_everything(commands);
        }
    }
}

#[derive(Component, Debug)]
pub struct Holder {
    held: bool,
}

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

#[derive(Debug, Resource, Default, Clone)]
pub struct PlayerPickupPoint {
    pub distance: f32,
    pub global_pos: Vec3,
}

#[derive(Component)]
pub struct HeldObject;

pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerPickupPoint>()
            .add_systems(Startup, setup_player_pickup_point)
            .add_systems(
                Update,
                (update_player_pickup_point, move_held_entity_to_hold).chain(),
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

pub fn move_held_entity_to_hold(
    pickup_point: Res<PlayerPickupPoint>,
    mut held: Query<&mut Transform, With<HeldObject>>,
    mut commands: Commands,
) {
    match held.get_single_mut() {
        Ok(mut transform) => transform.translation = pickup_point.global_pos,

        //err means more than one held object, so drop everything if that happens
        Err(_) => drop_everything(&mut commands),
    }
}

fn drop_everything(commands: &mut Commands<'_, '_>) {
    commands.add(move |world: &mut World| {
        let mut query = world.query_filtered::<Entity, With<HeldObject>>();
        let mut held_ids = query.iter_mut(world).collect::<Vec<Entity>>();
        for entity in held_ids.iter_mut() {
            world.entity_mut(*entity).remove::<HeldObject>();
        }
    })
}
