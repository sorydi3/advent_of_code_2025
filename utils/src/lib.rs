use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::{collections::VecDeque, fmt::Debug};

type Pos = (isize, isize); //(y,x)
type BoardLen = (usize, usize); //(y,x)

pub fn read_input(input_file_name: &str) -> std::io::Lines<BufReader<File>> {
    let file = File::open(input_file_name).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn write_input(input_file_name: &str) -> std::io::BufWriter<File> {
    use std::io::BufWriter;
    let file = File::create(input_file_name).unwrap();
    let writer = BufWriter::new(file);
    writer
}

pub enum Direction<T: Fn() -> bool> {
    Left((Pos, BoardLen, T)),
    Right((Pos, BoardLen, T)),
    Up((Pos, BoardLen, T)),
    Down((Pos, BoardLen, T)),
}

impl<T: Fn() -> bool> Debug for Direction<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left((pos, len, _)) => f.debug_tuple("Left").field(pos).field(len).finish(),
            Self::Right((pos, len, _)) => f.debug_tuple("Right").field(pos).field(len).finish(),
            Self::Up((pos, len, _)) => f.debug_tuple("Up").field(pos).field(len).finish(),
            Self::Down((pos, len, _)) => f.debug_tuple("Down").field(pos).field(len).finish(),
        }
    }
}

impl<T: Fn() -> bool> Direction<T> {
    fn is_valid_move(pot_move: Pos, len: BoardLen) -> bool {
        pot_move.1 < len.1 as isize
            && pot_move.1 >= 0isize
            && pot_move.0 < len.0 as isize
            && pot_move.0 >= 0
    }

    fn get_potencial_move(cur_pos: Pos, delta: &[i8; 2]) -> Pos {
        let [d_y, d_x] = delta;
        let pot_move = (
            cur_pos.0 as isize + *d_y as isize,
            cur_pos.1 as isize + *d_x as isize,
        );
        pot_move
    }

    pub fn check(&self) -> Option<Pos> {
        let validate = |delta: [i8; 2], pos: Pos, len: BoardLen| {
            let pot_move = Self::get_potencial_move(pos, &delta);
            match Self::is_valid_move(pot_move, len) {
                true => Some(pot_move),
                _ => None,
            }
        };

        match self {
            Self::Left((pos, len, _)) => {
                let delta = [0, -1]; //[y,x]
                validate(delta, *pos, *len).map(|pos| pos)
            }
            Self::Right((pos, len, _)) => {
                let delta = [0, 1]; //[y,x]

                validate(delta, *pos, *len).map(|pos| pos)
            }
            Self::Up((pos, len, _)) => {
                let delta = [-1, 0]; //[y,x]
                validate(delta, *pos, *len).map(|pos| pos)
            }
            Self::Down((pos, len, _)) => {
                let delta = [1, 0]; //[y,x]
                validate(delta, *pos, *len).map(|pos| pos)
            }
        }
    }
}
