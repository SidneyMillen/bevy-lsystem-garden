use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin, EguiSet, EguiUserTextures};

use crate::{
    fractal_plant::FractalPlant,
    lsys_rendering::FractalPlantUpdateEvent,
    lsystems::LSysDrawer,
    pickup::{Holder, HolderEvent, PickupDropEvent},
    player::ActiveEntity,
};

pub struct MyEguiPlugin;

impl Plugin for MyEguiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PanelOccupiedScreenSpace>()
            .add_plugins(EguiPlugin)
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
            .add_systems(
                PreUpdate,
                (test_side_and_top_panel, inspector_ui)
                    .chain()
                    .after(EguiSet::BeginFrame),
            );
    }
}

#[derive(Default, Resource, Debug, Clone)]
pub struct PanelOccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub trait SideMenuOptions<T: Event> {
    fn side_menu_options(
        &mut self,
        ui: &mut egui::Ui,
        event_writer: &mut EventWriter<T>,
        entity_id: Entity,
        active_id: Entity,
    );
}

pub fn test_side_and_top_panel(
    mut contexts: EguiContexts,
    mut occupied_space: ResMut<PanelOccupiedScreenSpace>,
    mut query: Query<(Entity, &mut FractalPlant)>,
    mut holder_query: Query<(Entity, &mut Holder)>,
    active_entity: ResMut<ActiveEntity>,
    mut event_writer: EventWriter<FractalPlantUpdateEvent>,
    mut holder_event_writer: EventWriter<PickupDropEvent>,
) {
    occupied_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Top Panel");
        })
        .response
        .rect
        .height();

    occupied_space.left = egui::SidePanel::left("side_panel")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Side Panel");

            for (entity, mut tree) in query.iter_mut() {
                match active_entity.id {
                    Some(id) => tree.side_menu_options(ui, &mut event_writer, entity, id),
                    None => {}
                }
            }
            for (entity, mut holder) in query.iter_mut() {
                holder.side_menu_options(ui, holder_event_writer, entity_id, active_id)
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

pub fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}
