use eframe::egui::{self, Color32, CornerRadius,Rect, RichText,Ui, Vec2, vec2};
use utils::read_input;
type BoardType = Vec<Vec<CellType>>;
#[derive(Debug, PartialEq)]
enum CellType {
    Roll,
    Taken((usize, Vec<[i8; 2]>)),
    Empty,
}
const DELTAS: [[i8; 2]; 8] = [[-1, 0],[1, 0],[0, -1],[0, 1],[-1, -1],[1, -1],[-1, 1],[1, 1],];
type Pos = (usize, usize);

#[derive(Default, Debug)]
struct Board {
    counter_rols: (usize,usize),
    total_takens: usize,
    board: BoardType,
    neigh:usize,
    roll_size:usize,
    filter_active:bool,
    step:usize
}

impl Board {
    pub fn new() -> Self {
        let board = Self {
            board: get_board(),
            counter_rols: (0,0),
            total_takens:0,
            neigh:4,
            roll_size:5,
            filter_active:false,
            step:0,
        };
        board
    }

    pub fn reset_board(&mut self) {
        self.board = get_board();
    }

    pub fn count_rolls(&mut self) {
         self.counter_rols.1=0;
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                match self.board[row][col]  {
                    CellType::Roll => {
                        self.counter_rols.1+=1;
                    },
                    _ => ()
                }
            }
        } 
    }

    pub fn filter_takens(&mut self) {
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                match self.board[row][col]  {
                    CellType::Taken((_, _)) => {
                        println!("found takent");
                        self.board[row][col] = CellType::Empty;
                    },
                    _ => ()
                }
            }
        } 
        println!("BOARD FILTERED");

        
    }

    fn is_there_is_a_roll(&mut self, cur_pos: Pos, delta: [i8; 2], len: (usize, usize)) -> bool {
        let [d_x, d_y] = delta;
        let pot_move = (
            cur_pos.0 as isize + d_y as isize,
            cur_pos.1 as isize + d_x as isize,
        );
        if !(pot_move.1 < len.1 as isize
            && pot_move.1 >= 0isize
            && pot_move.0 < len.0 as isize
            && pot_move.0 >= 0)
        { // check incalid pos
            return false;
        }
        match self.board[pot_move.0 as usize][pot_move.1 as usize] {
            CellType::Taken((_, _)) | CellType::Roll => true,
            _ => false,
        }
    }

    pub fn display_cell(&self, cell: &CellType, ui: &mut Ui) {
        let (response,mut painter) =
            ui.allocate_painter(egui::Vec2::splat(self.roll_size as f32), egui::Sense::click_and_drag());
        let mut rect: Rect = response.rect;
        let colors = (egui::Color32::RED,egui::Color32::LIGHT_GRAY,egui::Color32::LIGHT_GREEN);
        //painter.set_clip_rect(painter.clip_rect().intersect(rect));

        //painter.debug_rect(rect, color, text);
        match cell {
            CellType::Roll => {
                painter.rect(rect, CornerRadius::same(0), egui::Color32::RED /*format!("{}", ne)*/,(0.0, colors.0),egui::StrokeKind::Middle);
                painter.text(
                    rect.center(),
                    egui::Align2::LEFT_TOP,
                    format!(""),
                    egui::FontId::monospace(12.0),
                    egui::Color32::LIGHT_GRAY,
                );
            }
            CellType::Empty => {
                //painter.rect_filled(rect, egui::CornerRadius::same(5), egui::Color32::LIGHT_GRAY)
                painter.rect(rect, CornerRadius::same(0), egui::Color32::LIGHT_GRAY /*format!("{}", ne)*/,(1.0, colors.1),egui::StrokeKind::Middle);
                painter.text(
                    rect.center(),
                    egui::Align2::LEFT_TOP,
                    format!(""),
                    egui::FontId::monospace(12.0),
                    egui::Color32::LIGHT_GRAY,
                );
            }
            CellType::Taken((ne, neigth)) => {
                painter.rect(rect, CornerRadius::same(0), egui::Color32::LIGHT_GREEN /*format!("{}", ne)*/,(1.0, colors.2),egui::StrokeKind::Middle);
                painter.text(
                    rect.center(),
                    egui::Align2::LEFT_CENTER,
                    format!("{}", ne),
                    egui::FontId::monospace(12.0),
                    Color32::WHITE,
                );
                
            }
        };
    }

    fn display_board(&mut self, ui: &mut Ui) {
        egui::Grid::new("cellboard")
            .min_col_width(1.0)
            .spacing(vec2(0., 0.))
            .min_row_height(1.0)
            .show(ui, |ui| {
                for row in 0..self.board.len() {
                    for col in 0..self.board[0].len() {
                        let cell = &self.board[row][col];
                        self.display_cell(cell, ui);
                    }
                    ui.end_row(); // Advance to next row after each column
                }
            });
    }

    fn take_rolls(&mut self) {
        self.counter_rols.0 = 0;
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                if self.board[row][col] != CellType::Empty
                    && self.board[row][col] != CellType::Taken((0, [].into()))
                {
                    let mut counter_valids = 0;
                    let mut vec: Vec<[i8; 2]> = Vec::new();
                    for delta in DELTAS {
                        if self.is_there_is_a_roll(
                            (row, col),
                            delta,
                            (self.board.len(), self.board[0].len()),
                        ) {
                            counter_valids += 1;
                            vec.push(delta);
                        }
                    }
                    if counter_valids < self.neigh && self.board[row][col] != CellType::Empty {
                        self.counter_rols.0+=1;
                        self.board[row][col] = CellType::Taken((counter_valids, vec.clone()));

                    }
                }
            }
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                
                ui.add(egui::Checkbox::new(&mut self.filter_active, "Disable Table Reset"));
                if self.filter_active {
                    let button = ui.button("Step").labelled_by(ui.label(format!("{}",self.step)).id);
                    
                    ui.label(format!("Total: {}",self.total_takens));
                    if button.clicked() {
                        self.filter_takens();
                        self.step+=1;
                        self.total_takens+=self.counter_rols.0;
                    }
                }else {
                    self.total_takens = 0;
                    self.step=0;
                    self.reset_board();
                }
                if !(self.step ==0 && self.filter_active) {
                    self.take_rolls();
                }
                self.count_rolls();



                let rich_text_takens = RichText::new(format!("Num Rolls Taken: {}",self.counter_rols.0)).family(egui::FontFamily::Proportional).size(30.);
                let rich_text_rolls = RichText::new(format!("Num Rolls Not Taken: {}",self.counter_rols.1)).family(egui::FontFamily::Proportional).size(30.);

                ui.horizontal(|ui| {
                    ui.label(rich_text_takens);
                    ui.label(rich_text_rolls);
                });

                if !self.filter_active {
                    ui.add(egui::Slider::new(&mut self.neigh, 1..=100).text("Num Neightboors"));
                }
                ui.add(egui::Slider::new(&mut self.roll_size, 1..=100).text("Cell Size"));
                
                

                self.display_board(ui);

            });
        });
    }
}

fn main() {
    ui();
}

fn ui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(Vec2::splat(1000.)),
        ..Default::default()
    };
    eframe::run_native("Board", options, Box::new(|_| Ok(Box::new(Board::new()))))
        .expect("FAILED TO LUNCH THE UI!!");
}

fn get_board() -> Vec<Vec<CellType>> {
    let lines = read_input("./day4/input2.txt");
    lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '@' => CellType::Roll,
                    'x' => CellType::Taken((0, [].into())),
                    _ => CellType::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
