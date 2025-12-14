use core::f32;
use std::{collections::HashSet, io::Stdout};

use itertools::Itertools;
use ui::{
    App, CustomState, bottom_panel, central_panel, eframe,
    egui::{
        self, Align2, Color32, CornerRadius, FontId, IntoAtoms, Label, Pos2 as pos2, Rect, Stroke, StrokeKind, Ui, Vec2
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

type Machines = Vec<Vec<String>>;

#[derive(Clone, Debug)]

struct Canva {
    machines: Machines,
    combinations: Option<Vec<(Vec<String>, Vec<Vec<Vec<u32>>>, Vec<String>)>>,
    scale: f32,
    scene_rect: egui::Rect,
    threshold: f32,
    clusters: Vec<Vec<Pos2>>,
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            scene_rect: Rect::ZERO, // `egui::Scene` will initialize this to something valid
            machines: vec![],
            combinations: None,
            scale: f32::default(),
            threshold: f32::default(),
            clusters: vec![],
        }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let lines_input = read_input("./day10/input2.txt");
        let vec_machines: Vec<Vec<String>> = lines_input
            .map(|line| {
                line.map(|line| {
                    let machine = line
                        .split(" ")
                        .map(|str| str.to_string())
                        .collect::<Vec<String>>();
                    machine
                })
                .expect("INVALID COORDINATE FOUND!!")
            })
            .collect::<Vec<_>>();
        Self {
            //4294823708
            machines: vec_machines,
            ..Default::default()
        }
        .comb()
    }
}

impl Canva {
    fn compute_distance(&self, pos1: Pos2, pos2: Pos2) -> isize {
        let x: isize = (pos1.x - pos2.x).abs() + 1;
        let y = (pos1.y - pos2.y).abs() + 1;
        x * y
    }
    fn comb(mut self) -> Box<Self> {
        let comb = self.machines
            .iter()
            .map(|machine| {

                //println!("{:?}", machine);

                let lights = machine.first().unwrap().clone().chars().collect::<Vec<_>>();

                //println!("LIGHT: {:?}",lights);
                let joltages = machine.last().unwrap().chars().collect::<Vec<_>>();

                let mut aux = machine.clone();
                aux.remove(0);
                println!("{:?}",aux);
                aux.remove(aux.len()-1);

                //println!("AUX: {:?}",aux);

                let mut buttons = aux.iter().map(|button| {
                    button.clone().chars().filter(|c| !(c.eq(&'(') || c.eq(&')') || c.eq(&',') ))
                    .inspect(|c| println!("CHAR: {:?}",c))
                    .map(|cha| cha.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
                }).collect::<Vec<_>>();

                let mut comb:Vec<Vec<u32>> = Vec::new();

                let res = (2..buttons.len()).map(|k|{
                    buttons.iter().combinations(k).collect::<Vec<_>>()
                }).flatten().map(|v| {
                    let res = v.iter().map(|v| (**v).clone()).collect::<Vec<_>>();
                    res
                }).collect::<Vec<_>>();


                println!("RES {:?}",res);


                (lights,res,joltages)



            })
            .collect::<Vec<_>>();

        /*
        self.combinations = Some(
            self.machines
                .iter()
                .combinations(2)
                .map(|machines| {
                    (
                        (machines[0].clone(), machines[1].clone()),
                        self.compute_distance(machines[0].clone(), machines[1].clone()),
                    )
                })
                .sorted_by_key(|c| c.1)
                .collect::<Vec<_>>(),
        );

        println!(
            "PERMUTACIONS: {:?}",
            &self
                .combinations
                .as_ref()
                .unwrap()
                .iter()
                .map(|c| { c.1 })
                .collect::<Vec<_>>()
        );
        */

        //self.combinations=//Some(comb);

        Box::new(self)
    }
    fn set_permutacions(mut self) -> Box<Self> {
        /*
            self.combinations = Some(
                self.machines
                    .iter()
                    .enumerate()
                    .map(|first_coord| {
                        self.machines
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
                    .combinations
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|c| { c.1 })
                    .collect::<Vec<_>>()
            );
        */

        Box::new(self)
    }

    fn shapes_points(&self, ui: &mut Ui) {
        /*

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
                 */
            });
        });
    }
}

fn main() {
    let mut canva = Canva::new();

    //canva.get_clusters();

    //ui(*canva, "Canva");

    //println!("Canva: {:?}",canva);
}
