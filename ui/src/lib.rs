pub use eframe::App;
use eframe::{Frame, egui::Color32};
pub use eframe::egui;

use crate::egui::Vec2;
pub use eframe;
use eframe::egui::Ui;
use std::fmt::Debug;

pub trait CustomState {
    fn new() -> Box<Self>
    where
        Self: Sized;
}

pub fn ui<T: eframe::App + Default + Debug + CustomState + Sized>(state: T, ui_name: &str) -> () {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::splat(1000.)),
        ..Default::default()
    };
    let res = eframe::run_native("Board", options, Box::new(|_| Ok(T::new())))
        .expect("FAILED TO LUNCH THE UI!!");
}

/*
pub fn top_panel(ctx: &eframe::egui::Context) {
    egui::TopBottomPanel::top("TOP PANEL").show(ctx, |ui| {
             ui.label("Hello top");
    });
}
 */

pub fn top_panel<B>(ctx: &eframe::egui::Context, content: impl FnOnce(&mut Ui) -> B) {
    egui::TopBottomPanel::top("TOP PANEL").show(ctx, content);
}

pub fn central_panel<B>(ctx: &eframe::egui::Context, content: impl FnOnce(&mut Ui) -> B) {

    let my_custom_frame = egui::Frame{
        fill: Color32::WHITE,
        ..Default::default()
    };

    egui::CentralPanel::default().frame(my_custom_frame).show(ctx, content);
}

pub fn bottom_panel<B>(ctx: &eframe::egui::Context, content: impl FnOnce(&mut Ui) -> B) {
    egui::TopBottomPanel::bottom("BOTTOM PANEL").show(ctx, content);
}

#[cfg(test)]
mod tests {
    use super::*;
}
