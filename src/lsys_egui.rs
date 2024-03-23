use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiUserTextures};

use crate::{fractal_tree::FractalTree, lsystems::LSysDrawer};

#[derive(Default, Resource)]
pub struct PanelOccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub fn fractal_tree_ui(mut contexts: EguiContexts) {
    egui::Window::new("Fractal Tree").show(contexts.ctx_mut(), |ui| {
        ui.label("Fractal Tree");
    });
}

pub fn test_side_and_top_panel(
    mut contexts: EguiContexts,
    mut occupied_space: ResMut<PanelOccupiedScreenSpace>,
    mut query: Query<(&mut FractalTree, &mut LSysDrawer)>,
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

            for (mut fractal_tree, mut lsys_drawer) in query.iter_mut() {
                let old_start_pos = fractal_tree.start_pos;
                ui.horizontal(|ui| {
                    ui.label("Start Position");
                    ui.add(egui::Slider::new(&mut fractal_tree.start_pos.x, -0.5..=0.5));
                });

                let old_start_angle = fractal_tree.start_angle;
                ui.horizontal(|ui| {
                    ui.label("Start Angle");
                    ui.add(egui::Slider::new(
                        &mut fractal_tree.start_angle,
                        std::f32::consts::PI..=std::f32::consts::PI * -1.0,
                    ));
                });

                let old_line_length = fractal_tree.line_length;
                ui.horizontal(|ui| {
                    ui.label("Line Length");
                    ui.add(egui::Slider::new(&mut fractal_tree.line_length, 0.0..=1.0));
                });

                let old_iterations = fractal_tree.lsys.iterations;
                ui.horizontal(|ui| {
                    ui.label("Iterations");
                    ui.add(egui::Slider::new(&mut fractal_tree.lsys.iterations, 0..=10));
                });
                if old_start_pos != fractal_tree.start_pos
                    || old_start_angle != fractal_tree.start_angle
                    || old_line_length != fractal_tree.line_length
                    || old_iterations != fractal_tree.lsys.iterations
                {
                    lsys_drawer.changed = true;
                }
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
