use core::f32;
use glam::Vec3;
use std::collections::HashSet;

use itertools::Itertools;
use ui::{
    App, CustomState, bottom_panel, central_panel, eframe,
    egui::{self, Align2, Color32, FontId, Label, Pos2, Rect, Stroke, Ui, Vec2},
    top_panel, ui,
};

use utils::read_input;
/*
#[derive(Clone,Debug,Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
 */

type Jonctions = Vec<Vec3>;
#[derive(Clone, Debug)]

struct Canva {
    jonctions: Jonctions,
    permutations: Option<Vec<(((Vec3, usize), (Vec3, usize)), f32)>>,
    scale: f32,
    scene_rect: egui::Rect,
    threshold: f32,

    clusters: Vec<Vec<Vec3>>,
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO, // `egui::Scene` will initialize this to something valid
            jonctions: vec![],
            permutations: None,
            scale: f32::default(),
            threshold: f32::default(),
            clusters: vec![],
        }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let lines_input = read_input("./day8/input2.txt");
        let vec_pos_3d = lines_input
            .map(|line| {
                line.map(|line| {
                    let cordinates = line.split(",").collect::<Vec<_>>();
                    Vec3 {
                        x: cordinates[0].parse::<f32>().unwrap(),
                        y: cordinates[1].parse::<f32>().unwrap(),
                        z: cordinates[2].parse::<f32>().unwrap(),
                    }
                })
                .expect("INVALID COORDINATE FOUND!!")
            })
            .collect::<Vec<_>>();
        Self {
            jonctions: vec_pos_3d,
            ..Default::default()
        }
        .set_permutacions()
    }
}

impl Canva {
    fn set_permutacions(mut self) -> Box<Self> {
        self.permutations = Some(
            self.jonctions
                .iter()
                .enumerate()
                .map(|first_coord| {
                    self.jonctions
                        .iter()
                        .enumerate()
                        .skip(first_coord.0 + 1)
                        .map(|sec_coord| {
                            (
                                (
                                    (first_coord.1.clone(), first_coord.0),
                                    (sec_coord.1.clone(), sec_coord.0),
                                ),
                                first_coord.1.distance((*sec_coord.1).clone()),
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .sorted_by_key(|c| c.1 as u32)
                .collect::<Vec<_>>(),
        );
        Box::new(self)
    }

    fn get_cluster_v2(&self) {
        let mut groups: Vec<HashSet<usize>> = vec![];
        let mut counter = 0;
        'outer: for (((jonction1, id_jon_1), (jonction2, id_jon_2)), distance) in
            self.permutations.as_ref().unwrap().iter()
        {   
            if counter == 10 {
                break 'outer;
            }

            counter+=1;

            if groups.is_empty() {
                let mut set = HashSet::<usize>::new();
                set.insert(*id_jon_1);
                set.insert(*id_jon_2);
                groups.push(set);


                
            } else {
                if let Some(index) = groups.iter().position(|v| v.contains(id_jon_1)) {
                    if groups.iter().position(|v| v.contains(id_jon_2)).is_none() {
                        groups.get_mut(index).map(|set| {
                            set.insert(*id_jon_2);
                        });
                    }
                } else {
                    if let Some(index) = groups.iter().position(|v| v.contains(id_jon_2)) {
                        if groups.iter().position(|v| v.contains(id_jon_1)).is_none() {
                            groups.get_mut(index).map(|set| {
                                set.insert(*id_jon_1);
                            });
                        }
                    } else {
                        // when none of the jonction cannot be addet to a existing circuit
                        let mut set = HashSet::<usize>::new();
                        set.insert(*id_jon_1);
                        set.insert(*id_jon_2);
                        groups.push(set);
                    }
                }
            }
            println!("GROUP: {:?}",groups);
        }
        let res = groups.sort_by_key(|c|c.len());
        groups.reverse();
        println!("GROUPS: {:?}", groups);
        
    }

    fn get_clusters(&mut self) {
        let mut visited: Vec<bool> = vec![false; self.jonctions.len()];
        let mut groups: Vec<(Vec<usize>, Vec<Vec3>)> = vec![];

        let mut counter = 0u8;

        for (index_p1, point1) in self.jonctions.iter().enumerate() {
            if counter == 9 {
                break;
            }

            if visited[index_p1] {
                continue;
            }

            let j = index_p1;

            let mut distances: Vec<(f32, usize)> = vec![];

            visited[index_p1] = true;

            for (k, point2) in self.jonctions.iter().enumerate() {
                let distance = point1.distance(*point2);

                distances.push((distance, k));
            }

            println!(
                "DISTANCES: {:?}",
                distances.iter().map(|v| v.0).collect::<Vec<_>>()
            );

            distances.sort_by_key(|c| c.0 as i32);

            let min_jont = distances.iter().take(11).skip(1).next().unwrap();

            //println!("DISTANCES: {:?} MIN: {:?}",distances,min_jont);

            if visited[min_jont.1] {
                let res = groups
                    .iter_mut()
                    .find(|group| group.0.contains(&min_jont.1));

                res.map(|res| {
                    res.0.push(index_p1);
                    res.1.push(self.jonctions[index_p1]);
                });
            } else {
                // not visited

                println!("NOT VISITED!! {min_jont:?}");
                counter += 1;
                visited[min_jont.1] = true;
                groups.push((
                    vec![min_jont.1, index_p1],
                    vec![
                        self.jonctions[index_p1].clone(),
                        self.jonctions[min_jont.1].clone(),
                    ],
                ));
            }
        }

        let single_jonctio = visited
            .iter()
            .enumerate()
            .map(|(index, value)| if !value { Some([index]) } else { None })
            .collect::<Vec<_>>();

        println!(
            "GROUPS: {:?}",
            groups.iter().map(|v| v.0.clone()).collect::<Vec<_>>()
        );
        println!(
            "SINGLE GROUPS: {:?}",
            single_jonctio.iter().flatten().collect::<Vec<_>>()
        );
        println!("VISITED: {:?}", visited);
        /*
        println!(
            "Permutacions: {:?}. LEN: {:?}",
            self.permutations.iter().flatten().collect::<Vec<_>>(),
            self.permutations.iter().flatten().collect::<Vec<_>>().len()
        );
         */
    }

    fn shapes_points(&self, ui: &mut Ui) {
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
            .jonctions
            .iter()
            .enumerate()
            .map(|(i, point)| {
                let size = egui::Vec2::splat(2.0 * 8.);

                let point_in_screen = to_screen.transform_pos(
                    Pos2 {
                        x: point.x,
                        y: point.y,
                    } / self.scale,
                );

                println!("FROM: {}. IN SCREEN: {}", point, point_in_screen);

                let stroke = Stroke::new(1., Color32::RED);

                painter.circle(point_in_screen, 3., Color32::WHITE, stroke);
                painter.text(
                    point_in_screen,
                    Align2::LEFT_BOTTOM,
                    format!("{}", point / self.scale),
                    FontId::new(15., egui::FontFamily::Monospace),
                    Color32::BLACK,
                );
            })
            .collect::<Vec<_>>();
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

                    println!("SCREEN COORDINATES: {:?}", to_screen);

                    let _ = self
                        .jonctions
                        .iter()
                        .enumerate()
                        .map(|(i, point)| {
                            let size = egui::Vec2::splat(2.0 * 8.);

                            let point_in_screen = to_screen.transform_pos(
                                Pos2 {
                                    x: point.x,
                                    y: point.y,
                                } / self.scale,
                            );

                            println!("FROM: {}. IN SCREEN: {}", point, point_in_screen);

                            let stroke = Stroke::new(1., Color32::RED);

                            painter.circle(point_in_screen, 3., Color32::WHITE, stroke);
                            painter.text(
                                point_in_screen,
                                Align2::LEFT_BOTTOM,
                                format!("{}", point / self.scale),
                                FontId::new(15., egui::FontFamily::Monospace),
                                Color32::BLACK,
                            );
                        })
                        .collect::<Vec<_>>();
                })
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
    canva.get_cluster_v2();

    panic!("STOP");

    ui(*canva, "Canva");

    //println!("Canva: {:?}",canva);
}
