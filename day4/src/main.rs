use std::io::Empty;

use eframe::egui::{self, Color32, Ui, Vec2};
use utils::read_input;

type Board = Vec<Vec<CellType>>;

#[derive(Debug, PartialEq)]
enum CellType {
    Roll,
    Visited,
    Empty,
}

fn main() {
    let lines = read_input("./day4/input.txt");
    let mut ctx = egui::Context::default();
    ui();
    ////println!("{:?}", get_board());
}

const DELTAS: [[i8; 2]; 8] = [
    [-1, 0],
    [1, 0],
    [0, -1],
    [0, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
    [1, 1],
];
type Pos = (usize, usize);

fn valid_delta(cur_pos: Pos, delta: [i8; 2], len: (usize, usize), board: &Board) -> bool {
    //println!("DELTA: {:?} POS: {:?}", delta, cur_pos);
    let [d_x, d_y] = delta;

    let pot_move = (
        cur_pos.0 as isize + d_x as isize,
        cur_pos.1 as isize + d_y as isize,
    );
    println!("POT_MOVE: {:?}", pot_move);
    let res = pot_move.0 < len.0 as isize
        && pot_move.0 >= 0isize
        && pot_move.1 < len.1 as isize
        && pot_move.1 >= 0
        && board[pot_move.0 as usize][pot_move.1 as usize] != CellType::Roll;
    res
}



fn take_rolls(ui: &mut Ui,board : &mut Board) {
    let mut board_clone = board.iter().collect::<Vec<_>>();

    for row in board.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let mut counter_valids = 0;
            for delta in DELTAS {
                if valid_delta((col.0, row.0), delta, (board[0].len(), board.len()), board) {
                    counter_valids+=1;
                }

                if counter_valids < 4 {
                    board_clone[col.0][row.0] = CellType::Visited
                }
            }
        }
        ui.end_row(); // Advance to next row after each column
    }
}




fn display_board(ui: &mut Ui,board : &mut Board) {
    for row in board.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            let counter_valids = 0;
            for delta in DELTAS {
                //println!("CURRENT_POS: x: {:?} y:{:?}", col.0, row.0);
                if valid_delta((col.0, row.0), delta, (board.len(), board[0].len()), &board) {
                    let (response, painter) = ui
                        .allocate_painter(egui::Vec2::splat(100.0), egui::Sense::click_and_drag());
                    let rect = response.rect;
                    match &col.1 {
                        CellType::Roll => painter.rect_filled(
                            rect,
                            egui::CornerRadius::same(5),
                            egui::Color32::RED,
                        ),
                        CellType::Empty => painter.rect_filled(
                            rect,
                            egui::CornerRadius::same(5),
                            egui::Color32::LIGHT_GREEN,
                        ),
                        CellType::Visited => painter.rect_filled(
                            rect,
                            egui::CornerRadius::same(5),
                            egui::Color32::GREEN,
                        ),
                    };
                }
            }
        }
        ui.end_row(); // Advance to next row after each column
    }
}

fn ui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::splat(1000.)),
        ..Default::default()
    };

    eframe::run_simple_native("Board", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("cellboard")
                    .min_col_width(100.0)
                    .min_row_height(100.0)
                    .spacing(Vec2::splat(0.))
                    .show(ui, |ui| {
                        let mut board = get_board();
                        display_board(ui,&mut board)
                    });
            });
        });
    })
    .expect("FAILED TO LUNCH THE UI!!");
}

fn get_board() -> Vec<Vec<CellType>> {
    let lines = read_input("./day4/input.txt");
    lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '@' => CellType::Roll,
                    'x' => CellType::Visited,
                    _ => CellType::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
