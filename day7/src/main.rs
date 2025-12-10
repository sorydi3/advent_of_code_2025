use utils::Direction;
use utils::read_input;
type Board = Vec<Vec<char>>;

fn main() {
    let lines = read_input("../day7/input.txt");
    let mut board = lines
        .map(|line| {
            if let Ok(line) = line {
                line.chars().collect::<Vec<_>>()
            } else {
                panic!("NOT VALID LINE!!");
            }
        })
        .collect::<Vec<_>>();

    comput_board(&mut board);
    println!("COUNT: {:?}", count_split_times(&mut board))
}

fn count_split_times(board: &mut Board) -> usize {
    board
        .iter()
        .enumerate()
        .fold::<usize, _>(0, |acc, (row_index, row)| {
            let count = row.iter().enumerate().fold(0, |acc, (col_index, value)| {
                let up = Direction::Up((
                    (row_index as isize, col_index as isize),
                    (board.len(), board[0].len()),
                    || true,
                ))
                .check();

                match value {
                    '^' => {
                        if let Some((y, x)) = up {
                            if board[y as usize][x as usize] == '|' {
                                return acc + 1;
                            }
                        }
                        acc
                    }
                    _ => acc,
                }
            });

            acc + count
        })
}

fn comput_board(board: &mut Board) {
    let update_cell = |value: char, (y, x): (usize, usize), board: &mut Board| {
        board[y][x] = value;
    };
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let cell = &board[row][col];
            match cell {
                '.' => {
                    if let Some((y, x)) = Direction::Up((
                        (row as isize, col as isize),
                        (board.len(), board[0].len()),
                        || true,
                    ))
                    .check()
                    {
                        match board[y as usize][x as usize] {
                            'S' | '|' => {
                                board[row][col] = '|';
                            }
                            _ => (),
                        }
                    };
                }
                'S' => (),
                '^' => {
                    let left = Direction::Left((
                        (row as isize, col as isize),
                        (board.len(), board[0].len()),
                        || true,
                    ))
                    .check();
                    let right = Direction::Right((
                        (row as isize, col as isize),
                        (board.len(), board[0].len()),
                        || true,
                    ))
                    .check();

                    if let Some((y, x)) = left {
                        let cell = board[y as usize][x as usize];
                        if cell == '.' {
                            update_cell('|', (y as usize, x as usize), board)
                        }
                    };

                    if let Some((y, x)) = right {
                        let cell = board[y as usize][x as usize];
                        if cell == '.' {
                            update_cell('|', (y as usize, x as usize), board)
                        }
                    };
                }
                '|' => {
                    let up = Direction::Up((
                        (row as isize, col as isize),
                        (board.len(), board[0].len()),
                        || true,
                    ))
                    .check();
                    if let Some((y, x)) = up {
                        let cell = board[y as usize][x as usize];
                        if cell == '.' {
                            update_cell('|', (row, col), board)
                        }
                    };
                }
                _ => panic!("NOT VALID CHAR"),
            }
        }
    }
}
