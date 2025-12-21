use itertools::Itertools;
use ui::{CustomState, eframe::glow::VENDOR};
use utils::read_input;




type Board = Vec<Vec<char>>;
#[derive(Clone, Debug)]
struct Shape {
    pub id: u8,
    shape: Board,
}


impl Default for Shape {
    fn default() -> Self {
        Self {
            id: u8::default(),
            shape: vec![vec!['.'; 3]; 3],
        }
    }
}

impl Shape {
    fn new(id: u8, shape: Board) -> Self {
        Self { id, shape }
    }
    fn rotate(&self) -> Self {
        todo!()
    }
    fn reflecte(&self) -> Self {
        todo!()
    }
    pub fn shapes(&self) -> Vec<Shape> {
        //only rotate
        // only reflecte
        // rotate and reflecte
        // reflecte and rotate
        vec![Shape::default(); 4]
    }
}

#[derive(Clone, Debug)]
struct Region {
    shape_count: Vec<usize>,
    region: Board,
}

impl Region {
    fn new(region: Board, shape_count: Vec<usize>) -> Self {
        Self {
            shape_count,
            region,
        }
    }
}

#[derive(Clone, Debug)]
struct Canva {
    counter: u8,
    regions: Vec<Region>,
    shapes: Vec<Shape>,
}

impl Default for Canva {
    fn default() -> Self {
        Self {
            regions: vec![],
            shapes: vec![],
            counter: 0,
        }
    }
}

use itertools::Chunk;
use std::{path::Display, slice::Iter};

impl Canva {
    fn process_chunk(
        chunk: Chunk<'_, Iter<'_, String>>,
        shapes: &mut Vec<Shape>,
        regions: &mut Vec<Region>,
    ) -> () {
        //let res = chunk.map(|c| println!("LINE :{:?} ",c)).collect::<Vec<_>>();

        let mut chunk = chunk;

        if let Ok(chape_id) = chunk
            .next()
            .as_ref()
            .clone()
            .unwrap()
            .trim_matches(':')
            .parse::<u8>()
        {
            let mut board: Board = Board::new();
            while let Some(line) = chunk.next() {
                board.push(line.chars().collect());
            }
            let shape = Shape::new(chape_id, board);
            println!("SHAPE: {:?}", shape);
            shapes.push(shape);
        } else {
            chunk.for_each(|region| {
                if let Some((board_size, shape_count)) = region.split_once(":") {
                    if let Some((x, y)) = board_size.split_once("x") {
                        let x = x.parse::<usize>().unwrap();
                        let y = y.parse::<usize>().unwrap();
                        let board: Vec<Vec<char>> = vec![vec!['.'; x]; y];
                        let shapes_count = shape_count
                            .trim()
                            .split_whitespace()
                            .map(|c| c.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();

                        let region: Region = Region::new(board, shapes_count);
                        println!("REGION:: {:?}", region);

                        regions.push(region);
                    }
                }
            });
        }
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let mut lines = read_input("./day12/input2.txt");

        let res = lines
            .filter(|line| !line.as_ref().unwrap().eq(""))
            .map(|line| line.unwrap())
            .collect::<Vec<_>>();

        let chunks = res.iter().chunks(4);
        let mut shapes: Vec<Shape> = vec![];
        let mut regions: Vec<Region> = vec![];
        for chu in chunks.into_iter() {
            println!("CHUNK:-----");

            //let mut chup =  chu;
            Canva::process_chunk(chu, &mut shapes, &mut regions);
        }
        panic!("HERE");
    }
}

fn main() {
    let canva = Canva::new();
    println!("Hello, world!");
}
