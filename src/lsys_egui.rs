use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin, EguiSet, EguiUserTextures};

use crate::{
    fractal_plant::FractalPlant, lsys_rendering::FractalPlantUpdateEvent, lsystems::LSysDrawer,
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
    );
}

pub fn test_side_and_top_panel(
    mut contexts: EguiContexts,
    mut occupied_space: ResMut<PanelOccupiedScreenSpace>,
    mut query: Query<(Entity, &mut FractalPlant)>,
    mut event_writer: EventWriter<FractalPlantUpdateEvent>,
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
                tree.side_menu_options(ui, &mut event_writer, entity);
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
