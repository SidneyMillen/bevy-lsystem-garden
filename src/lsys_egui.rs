use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin, EguiUserTextures};

use crate::{fractal_plant::FractalPlant, lsystems::LSysDrawer};

#[derive(Default, Resource, Debug, Clone)]
pub struct PanelOccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub trait SideMenuOptions {
    fn side_menu_options(&mut self, drawer: &mut LSysDrawer, ui: &mut egui::Ui);
}

pub fn test_side_and_top_panel(
    mut contexts: EguiContexts,
    mut occupied_space: ResMut<PanelOccupiedScreenSpace>,
    mut query: Query<(&mut FractalPlant, &mut LSysDrawer)>,
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

            for (mut tree, mut drawer) in &mut query {
                tree.side_menu_options(&mut drawer, ui);
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
