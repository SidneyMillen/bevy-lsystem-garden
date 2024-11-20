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
    pickup::{ActiveEntityCandidate, Holder},
    player::ActiveEntity,
    GameState,
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

pub trait SideMenuOptions {
    fn side_menu_options(
        &mut self,
        ui: &mut egui::Ui,
        id: Entity,
        commands: &mut Commands,
        gs: &mut NextState<GameState>,
    );
}

pub fn test_side_and_top_panel(
    mut contexts: EguiContexts,
    mut occupied_space: ResMut<PanelOccupiedScreenSpace>,
    mut query: Query<(Entity, &mut FractalPlant)>,
    mut active_candidate_query: Query<(Entity, &mut ActiveEntityCandidate)>,
    active_entity: ResMut<ActiveEntity>,
    mut commands: Commands,
    mut gs: ResMut<NextState<GameState>>,
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
                match active_entity.id == Some(entity) {
                    true => tree.side_menu_options(ui, entity, &mut commands, &mut gs),
                    false => {}
                }
            }

            for (entity, mut obj) in active_candidate_query.iter_mut() {
                match active_entity.id == Some(entity) {
                    true => obj.side_menu_options(ui, entity, &mut commands, &mut gs),
                    false => {}
                }
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

    egui::Window::new("DEBUG").show(egui_context.get_mut(), |ui| {
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
