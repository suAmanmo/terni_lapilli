use std::fmt;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => " ",
                Cell::X => "X",
                Cell::O => "O",
            }
        )
    }
}

type Grid = [[Cell; 3]; 3];

struct Game {
    grid: Grid,
    current_player: Cell,
    moves: Vec<(usize, usize)>,
}

impl Game {
    fn new() -> Self {
        Game {
            grid: [[Cell::Empty; 3]; 3],
            current_player: Cell::X,
            moves: Vec::new(),
        }
    }

    fn play(&mut self) {
        loop {
            self.display();
            self.make_move();
            if self.check_win() {
                self.display();
                println!("Player {} wins!", self.current_player);
                break;
            } else if self.check_draw() {
                self.display();
                println!("It's a draw!");
                break;
            }
            self.switch_player();
        }
    }

    fn display(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{} | ", cell);
            }
            println!("\n----------");
        }
    }

    fn make_move(&mut self) {
        loop {
            println!("Player {}, enter your move (row col): ", self.current_player);
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim().split_whitespace().map(|num| num.parse()).collect::<Result<Vec<usize>, _>>() {
                Ok(input) if input.len() == 2 && input[0] > 0 && input[0] <= 3 && input[1] > 0 && input[1] <= 3 => {
                    let row = input[0] - 1;
                    let col = input[1] - 1;
                    if self.grid[row][col] == Cell::Empty {
                        self.grid[row][col] = self.current_player;
                        self.moves.push((row, col));
                        if self.moves.len() > 6 {
                            let (old_row, old_col) = self.moves.remove(0);
                            self.grid[old_row][old_col] = Cell::Empty;
                        }
                        break;
                    } else {
                        println!("Invalid move! The cell is not empty.");
                    }
                },
                _ => println!("Invalid input! Please enter two numbers between 1 and 3, separated by a space."),
            }
        }
    }

    fn check_win(&self) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1)];
        for row in 0..3 {
            for col in 0..3 {
                if self.grid[row][col] != Cell::Empty
                    && directions
                        .iter()
                        .any(|&(dr, dc)| self.check_line(row, col, dr, dc))
                {
                    return true;
                }
            }
        }
        false
    }

    fn check_line(&self, row: usize, col: usize, dr: usize, dc: usize) -> bool {
        let (mut r, mut c) = (row, col);
        for _ in 0..2 {
            r += dr;
            c += dc;
            if r >= 3 || c >= 3 || self.grid[r][c] != self.grid[row][col] {
                return false;
            }
        }
        true
    }

    fn check_draw(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == Cell::X {
            Cell::O
        } else {
            Cell::X
        };
    }
}

fn main() {
    let mut game = Game::new();
    game.play();
}
