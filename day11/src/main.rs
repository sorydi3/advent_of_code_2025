use std::{path, usize};

use itertools::Itertools;
use ui::{App, CustomState, central_panel, eframe};

use utils::read_input;

type Node = (String, Vec<String>);

type Nodes = Vec<Node>;

#[derive(Clone, Debug)]
struct Canva {
    nodes: Nodes,
}

impl Default for Canva {
    fn default() -> Self {
        Self { nodes: vec![] }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let nodes = read_input("./day11/input.txt");
        let vec_nodes = nodes
            .map(|line| {
                line.map(|line| {
                    let node = line
                        .split_once(":")
                        .map(|(device_name, outputs)| {
                            let node_name: String = device_name.trim().to_string();
                            let outputs = outputs
                                .split(" ")
                                .map(|edge| edge.trim().to_string())
                                .filter(|edge| !edge.eq(""))
                                .collect::<Vec<_>>();

                            (node_name, outputs)
                        })
                        .expect("MMM");

                    node
                })
                .expect("MMMM")
            })
            .collect::<Vec<_>>();

        let res = Self {
            nodes: vec_nodes,
            ..Default::default()
        };
        Box::new(res)
    }
}

impl Canva {
    fn get_paths(&self) {
        let mut visited: Vec<String> = vec![];
        let init_node = self.nodes.get(1).unwrap();
        let mut paths: Vec<Vec<String>> = vec![];
        visited.push(init_node.0.clone());
        self.i_get_path(init_node, &mut visited, &mut paths);

        println!("NUM_PATHS: {}", paths.iter().count());

        println!("PATHS: {:?}",paths);
    }

    fn get_pos_candidate(&self, candidate_name: &str) -> Option<usize> {
        self.nodes
            .iter()
            .position(|(node_name, _)| node_name.eq(candidate_name))
    }

    fn i_get_path(
        &self,
        (_, candidates): &(String, Vec<String>),
        visited: &mut std::vec::Vec<String>,
        paths: &mut Vec<std::vec::Vec<String>>,
    ) {

        println!("doing... {:?}",candidates);
        for candidate in candidates.iter() {
            if candidate.eq("out") {
                visited.push(candidate.clone());
                paths.push(visited.clone());
                visited.pop();
            } else {
                let pos_cand = self
                    .get_pos_candidate(&candidate)
                    .expect("POS CANDIDATE NOT FOUND");
                if !visited.contains(candidate) {
                    visited.push(candidate.clone());
                    self.i_get_path(self.nodes.get(pos_cand).unwrap(), visited, paths);
                    visited.pop();
                }
            }
        }
    }
}

impl App for Canva {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        /*
         */
    }
}

fn main() {
    let mut canva = Canva::new();
    canva.get_paths();

    // println!("MIN: {:?}", res);
}
