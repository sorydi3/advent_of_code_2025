use itertools::Itertools;
use ui::{CustomState, eframe::glow::VENDOR};
use utils::read_input;

type Board = Vec<Vec<char>>;
#[derive(Clone, Debug)]
struct Shape {
    pub id: u8,
    shape: Board,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "").expect("mm");
        self.shape.iter().for_each(|line| {
            line.iter().for_each(|char| {
                write!(f, " {} ", char).expect("mmm");
            });
            writeln!(f, "").expect("mm");
        });

        Ok(())
    }
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

    fn area(&self) -> usize {
        self.shape[0].len() * self.shape.len()
    }

    fn shapes_area(shapes: Vec<Self>) -> usize {
        shapes.iter().fold(0, |acc, shape| acc + shape.area())
    }

    pub fn tails_count(shapes: Vec<Self>) -> usize {
        shapes
            .iter()
            .fold(0, |acc, shape| acc + shape.count() as usize)
    }

    pub fn count(&self) -> u8 {
        self.shape.iter().fold(0, |acc, row| {
            row.iter().filter(|c| (*c).eq(&'#')).count() as u8 + acc
        })
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

    fn total_shape_count_are_naive(&self, shapes: &Vec<Shape>) -> (usize, usize) {
        let mut count_tailes = 0;
        let sum = self
            .shape_count
            .iter()
            .enumerate()
            .map(|count| {
                let res = count.1 * 9;
                if res > 0 {
                    count_tailes = count_tailes + (count.1 * shapes[count.0].count() as usize);
                }

                res
            })
            .sum::<usize>();
        (sum, count_tailes)
    }

    fn check(&self, shapes: &Vec<Shape>) -> bool {
        let region_area = self.area();
        let (sum_naive, tails_count) = self.total_shape_count_are_naive(shapes);
        sum_naive <= region_area || tails_count <= region_area
    }

    fn area(&self) -> usize {
        self.region[0].len() * self.region.len()
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "").expect("mm");
        self.region.iter().for_each(|line| {
            line.iter().for_each(|char| {
                write!(f, " {} ", char).expect("mmm");
            });
            writeln!(f, "").expect("mm");
        });

        Ok(())
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

use core::fmt;
use itertools::Chunk;
use std::{path::Display, slice::Iter};

impl Canva {
    fn process_chunk(
        chunk: Chunk<'_, Iter<'_, String>>,
        shapes: &mut Vec<Shape>,
        regions: &mut Vec<Region>,
    ) -> () {
        //let res = chunk.map(|c| println!("LINE :{:?} ",c)).collect::<Vec<_>>();

        let mut chunk = chunk.peekable();

        if let Ok(chape_id) = chunk
            .peek()
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
            //println!("SHAPE: {}", shape);
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
                        //println!("REGION:: {}", region);

                        regions.push(region);
                    }
                }
            });
        }
    }
    fn check_regions(&self) -> i32 {
        let mut counter = 0;
        self.regions.iter().for_each(|region| {
            if region.check(&self.shapes) {
                counter += 1;
            }
        });
        counter
    }
}

impl CustomState for Canva {
    fn new() -> Box<Self> {
        let mut lines = read_input("./day12/input.txt");

        let res = lines
            .filter(|line| !line.as_ref().unwrap().eq(""))
            .map(|line| line.unwrap())
            .collect::<Vec<_>>();

        let chunks = res.iter().chunks(4);

        let mut shapes: Vec<Shape> = vec![];
        let mut regions: Vec<Region> = vec![];
        for chu in chunks.into_iter().enumerate() {
            //let mut chup =  chu;
            Canva::process_chunk(chu.1, &mut shapes, &mut regions);
        }
        Box::new(Self {
            counter: 0,
            regions,
            shapes,
        })
    }
}

fn main() {
    let canva = Canva::new();
    println!("COUNT: {}", canva.check_regions());
}
