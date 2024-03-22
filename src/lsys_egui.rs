use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PresentMode, PrimaryWindow, WindowRef, WindowResolution},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiUserTextures};

use crate::fractal_tree::FractalTree;

pub fn fractal_tree_ui(mut contexts: EguiContexts) {
    egui::Window::new("Fractal Tree").show(contexts.ctx_mut(), |ui| {
        ui.label("Fractal Tree");
    });
}
