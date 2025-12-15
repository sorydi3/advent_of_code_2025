use std::usize;

use itertools::Itertools;
use ui::{App, CustomState, central_panel, eframe};

use utils::read_input;

type Machines = Vec<Vec<String>>;

#[derive(Clone, Debug)]
struct Canva {
    machines: Machines,
    combinations: Option<Vec<(Vec<char>, Vec<Vec<Vec<u32>>>, Vec<char>)>>,
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            machines: vec![],
            combinations: None,
        }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let lines_input = read_input("./day10/input.txt");
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
    fn minimize(&self) -> usize {
        self.combinations
            .as_ref()
            .unwrap()
            .iter()
            .fold::<usize, _>(0usize, |acc, machine| {
                let min = self.find_min_press_buttons(machine);
                acc + min
            })
    }

    fn update_state(&self, state: &Vec<char>, comb: &[Vec<u32>]) -> Vec<char> {
        let mut empty = vec!['.'; state.len()];
        comb.iter().for_each(|combi| {
            combi
                .iter()
                .for_each(|state_index| match empty[*state_index as usize] {
                    '.' => empty[*state_index as usize] = '#',
                    '#' => empty[*state_index as usize] = '.',
                    _ => panic!("MMM: {:?}", empty[*state_index as usize]),
                });
        });
        empty
    }

    fn find_min_press_buttons(
        &self,
        machine: &(Vec<char>, Vec<Vec<Vec<u32>>>, Vec<char>),
    ) -> usize {
        let (state, comb, _) = machine;
        let state = state.clone();
        let mut min = usize::MAX;
        'outer_loop: for combination in comb.iter() {
            let updated_state = self.update_state(&state, &combination[..]);
            if updated_state.iter().eq(state.iter()) {
                min = combination.len().min(min.into()) as usize;
                break 'outer_loop;
            }
        }
        min
    }

    fn comb(mut self) -> Box<Self> {
        let comb = self
            .machines
            .iter()
            .map(|machine| {
                let lights = machine.first().unwrap().clone().chars().collect::<Vec<_>>();

                println!("LIGHTTTTTT: {:?}", lights);
                let joltages = machine.last().unwrap().chars().collect::<Vec<_>>();

                let mut aux = machine.clone();
                aux.remove(0);
                aux.remove(aux.len() - 1);
                let buttons = aux
                    .iter()
                    .map(|button| {
                        button
                            .clone()
                            .chars()
                            .filter(|c| !(c.eq(&'(') || c.eq(&')') || c.eq(&',')))
                            .map(|cha| cha.to_digit(10).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                let mut comb: Vec<Vec<u32>> = Vec::new();

                let res = (1..buttons.len())
                    .map(|k| buttons.iter().combinations(k).collect::<Vec<_>>())
                    .flatten()
                    .map(|v| {
                        let res = v.iter().map(|v| (**v).clone()).collect::<Vec<_>>();
                        res
                    })
                    .collect::<Vec<_>>();
                (
                    lights
                        .iter()
                        .filter(|c| !((**c).eq(&'[') || (**c).eq(&']')))
                        .map(|c| c.clone())
                        .collect::<Vec<_>>(),
                    res,
                    joltages,
                )
            })
            .collect::<Vec<_>>();
        self.combinations = Some(comb);
        Box::new(self)
    }
}

impl App for Canva {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {

        /*

        top_panel(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {


                //ui.add(egui::Slider::new(&mut self.scale, 0.0..=1000.0).text("My value"));
            });
        });
        central_panel(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                /*
                 */
            });
        });
         */
    }
}

fn main() {
    let mut canva = Canva::new();

    let res = canva.minimize();

    println!("MIN: {:?}", res);
}
