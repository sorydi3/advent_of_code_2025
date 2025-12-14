use core::f32;
use std::collections::HashSet;

use itertools::Itertools;
use ui::{
    App, CustomState, bottom_panel, central_panel, eframe,
    egui::{
        self, Align2, Color32, CornerRadius, FontId, Label, Pos2 as pos2, Rect, Stroke, StrokeKind,
        Ui, Vec2,
    },
    top_panel, ui,
};

use utils::read_input;
/*
#[derive(Clone,Debug,Default)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
 */
#[derive(Clone, Debug)]
struct Pos2 {
    pub x: isize,
    pub y: isize,
}

type Cordinates = Vec<Pos2>;

#[derive(Clone, Debug)]

struct Canva {
    coordinates: Cordinates,
    permutations: Option<Vec<((Pos2, Pos2), isize)>>,
    scale: f32,
    scene_rect: egui::Rect,
    threshold: f32,
    clusters: Vec<Vec<Pos2>>,
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO, // `egui::Scene` will initialize this to something valid
            coordinates: vec![],
            permutations: None,
            scale: f32::default(),
            threshold: f32::default(),
            clusters: vec![],
        }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let lines_input = read_input("./day9/input.txt");
        let vec_pos_2d = lines_input
            .map(|line| {
                line.map(|line| {
                    let cordinates = line.split(",").collect::<Vec<_>>();
                    Pos2 {
                        x: cordinates[0].parse::<isize>().unwrap(),
                        y: cordinates[1].parse::<isize>().unwrap(),
                    }
                })
                .expect("INVALID COORDINATE FOUND!!")
            })
            .collect::<Vec<_>>();
        Self {
            //4294823708
            coordinates: vec_pos_2d,
            ..Default::default()
        }
        .perm()
    }
}

impl Canva {
    fn compute_distance(&self, pos1: Pos2, pos2: Pos2) -> isize {
        let x: isize = (pos1.x - pos2.x).abs() + 1;
        let y = (pos1.y - pos2.y).abs() + 1;
        x * y
    }
    fn perm(mut self) -> Box<Self> {
        self.permutations = Some(
            self.coordinates
                .iter()
                .permutations(2)
                .map(|coordinates| {
                    (
                        (coordinates[0].clone(), coordinates[1].clone()),
                        self.compute_distance(coordinates[0].clone(), coordinates[1].clone()),
                    )
                })
                .sorted_by_key(|c| c.1)
                .collect::<Vec<_>>(),
        );

        println!(
            "PERMUTACIONS: {:?}",
            &self
                .permutations
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| { c.1 })
                .collect::<Vec<_>>()
        );

        Box::new(self)
    }
    fn set_permutacions(mut self) -> Box<Self> {
        self.permutations = Some(
            self.coordinates
                .iter()
                .enumerate()
                .map(|first_coord| {
                    self.coordinates
                        .iter()
                        .enumerate()
                        .map(|sec_coord| {
                            (
                                (first_coord.1.clone(), sec_coord.1.clone()),
                                self.compute_distance(first_coord.1.clone(), sec_coord.1.clone()),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .sorted_by_key(|c| c.1 as u32)
                .collect::<Vec<_>>(),
        );

        println!(
            "PERMUTACIONS: {:?}",
            &self
                .permutations
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| { c.1 })
                .collect::<Vec<_>>()
        );

        Box::new(self)
    }

    fn shapes_points(&self, ui: &mut Ui) {

        /*
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense::hover(),
            );

            let to_screen = egui::emath::RectTransform::from_to(
                egui::Rect::from_min_size(Pos2::ZERO, Vec2::splat(1000.)),
                response.rect,
            );

            //painter.rect(rect, corner_radius, fill_color, stroke, stroke_kind)

            println!("SCREEN COORDINATES: {:?}", to_screen);

            let _ = self
                .permutations
                .as_ref()
                .unwrap()
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let size = egui::Vec2::splat(2.0 * 8.);
                    let ((pos1, pos2), _) = point;
                    let point_in_screen_sup = to_screen.transform_pos(
                        Pos2 {
                            x: pos1.x,
                            y: pos1.y,
                        } / self.scale,
                    );

                    let point_in_screen_inf = to_screen.transform_pos(
                        Pos2 {
                            x: pos2.x,
                            y: pos2.y,
                        } / self.scale,
                    );

                    //println!("FROM: {}. IN SCREEN: {}", point, point_in_screen);

                    let stroke = Stroke::new(1., Color32::BLACK);
                    let rec = Rect::from_min_max(point_in_screen_sup, point_in_screen_inf);
                    painter.rect(
                        rec.clone(),
                        CornerRadius::same(3),
                        Color32::LIGHT_GREEN,
                        stroke,
                        StrokeKind::Inside,
                    );
                    painter.text(
                        rec.center(),
                        Align2::CENTER_CENTER,
                        format!("{}", point_in_screen_sup),
                        FontId::new(15., egui::FontFamily::Monospace),
                        Color32::BLACK,
                    );
                })
                .collect::<Vec<_>>();
        }
        */
    }
}

impl App for Canva {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        top_panel(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::Slider::new(&mut self.scale, 0.0..=1000.0).text("My value"));
            });
        });
        central_panel(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                /*
                //let mut scene = egui::Scene::new().zoom_range(0.0..=f32::INFINITY)
                let scene = egui::Scene::new()
                    .max_inner_size([ui.available_width(), 1000.])
                    .zoom_range(0.1..=2.0);
                scene.show(ui, &mut self.scene_rect, |ui| {
                    //self.shapes_points(ui);

                    let (response, painter) = ui.allocate_painter(
                        egui::Vec2::new(ui.available_width(), ui.available_height()),
                        egui::Sense::hover(),
                    );

                    let to_screen = egui::emath::RectTransform::from_to(
                        egui::Rect::from_min_size(Pos2::ZERO, Vec2::splat(1000.)),
                        response.rect,
                    );

                    //painter.rect(rect, corner_radius, fill_color, stroke, stroke_kind)

                    //println!("SCREEN COORDINATES: {:?}", to_screen);

                    let _ = self
                        .permutations
                        .as_ref()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(|(i, point)| {
                            let size = egui::Vec2::splat(2.0 * 8.);
                            let ((pos1, pos2), _) = point;

                            let point_in_screen_sup = to_screen.transform_pos(
                                Pos2 {
                                    x: pos1.x,
                                    y: pos1.y,
                                } / self.scale,
                            );

                            let point_in_screen_inf = to_screen.transform_pos(
                                Pos2 {
                                    x: pos2.x,
                                    y: pos2.y,
                                } / self.scale,
                            );

                            //println!("p1{} p2: {}", point_in_screen_sup, point_in_screen_inf);

                            let stroke = Stroke::new(1., Color32::BLACK);
                            let rec = Rect::from_min_max(point_in_screen_sup, point_in_screen_inf);
                            painter.rect(
                                rec.clone(),
                                CornerRadius::same(3),
                                Color32::LIGHT_GREEN,
                                stroke,
                                StrokeKind::Inside,
                            );
                            painter.text(
                                rec.center(),
                                Align2::CENTER_CENTER,
                                format!("{}", rec.area()),
                                FontId::new(15., egui::FontFamily::Monospace),
                                Color32::BLACK,
                            );
                        })
                        .collect::<Vec<_>>();
                })

                */
            });
        });
        /*
             bottom_panel(ctx, |ui| {
             egui::ScrollArea::vertical().show(ui, |ui| {
                 ui.add(Label::new("BOTTOM PANEL"));
             });
         });
        */
    }
}

fn main() {
    let mut canva = Canva::new();

    //canva.get_clusters();

    //ui(*canva, "Canva");

    //println!("Canva: {:?}",canva);
}
