use std::io;

use rand::Rng;
use rand::rngs::ThreadRng;

use std::{thread, time};
use std::time::Duration;

// ----------------------------------------------------------------------------
struct Point {
    row: usize,
    col: usize
}

#[derive(Debug, PartialEq)] // para que se pueda imprimir y comparar
enum WinState { PLAYER1, PLAYER2, TIE, CONTINUE }

static P1: char = '⚫';
static P2: char = '🔴';

static ONE_SEC: Duration = time::Duration::from_millis(1000);

// ----------------------------------------------------------------------------
fn set_first_player(rng: &mut ThreadRng, current_player: &mut char) {
    if rng.gen_range(0..2) == 0 { // (0..2) between 0 and 1
        *current_player = P1;
    } else {
        *current_player = P2;
    }
}

fn get_winner_by_str(s: String) -> WinState {
    if s == format!("{x}{x}{x}", x = P1) {
        return WinState::PLAYER1;
    } else if s == format!("{x}{x}{x}", x = P2) {
        return WinState::PLAYER2;
    }
    return WinState::CONTINUE;
}

fn get_winner(state: &[[char; 3]; 3]) -> WinState {
    // rows
    for row in state {
        let row_str: String = row.iter().collect();
        let winner = get_winner_by_str(row_str);
        if winner != WinState::CONTINUE {
            return winner;
        }
    }
    // cols
    for col_num in 0..3 {
        let col_str: String = format!("{}{}{}", state[0][col_num], state[1][col_num], state[2][col_num]);
        let winner = get_winner_by_str(col_str);
        if winner != WinState::CONTINUE {
            return winner;
        }
    }
    // diagonals
    let left_diagonal_str = format!("{}{}{}", state[0][0], state[1][1], state[2][2]);
    let winner = get_winner_by_str(left_diagonal_str);
    if winner != WinState::CONTINUE {
        return winner;
    }
    let right_diagonal_str = format!("{}{}{}", state[0][2], state[1][1], state[2][0]);
    let winner = get_winner_by_str(right_diagonal_str);
    if winner != WinState::CONTINUE {
        return winner;
    }
    // check open spots
    let mut open_spots = 0;
    for row in state {
        for col in row {
            if *col == '⬜' {
                open_spots += 1;
            }
        }
    }
    if open_spots == 0 {
        return WinState::TIE;
    }
    // continue
    return WinState::CONTINUE;
}

fn print_board(state: &[[char; 3]; 3]) {
    println!("⬛1️⃣ 2️⃣ 3️⃣ ⬛");
    println!("1️⃣ {}{}{}1️⃣", state[0][0], state[0][1], state[0][2]);
    println!("2️⃣ {}{}{}2️⃣", state[1][0], state[1][1], state[1][2]);
    println!("3️⃣ {}{}{}3️⃣", state[2][0], state[2][1], state[2][2]);
    println!("⬛1️⃣ 2️⃣ 3️⃣ ⬛");
}

fn get_next_position() -> Point {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut new_row: usize = line.chars().nth(0).unwrap() as usize - 0x30;
    let mut new_col: usize = line.chars().nth(1).unwrap() as usize - 0x30;
    while new_row > 3 || new_col > 3 {
        eprintln!("Bad input!");
        line = "".to_string();
        io::stdin().read_line(&mut line).unwrap();
        new_row = line.chars().nth(0).unwrap() as usize - 0x30;
        new_col = line.chars().nth(1).unwrap() as usize - 0x30;
    }
    return Point {
        row: new_row - 1,
        col: new_col - 1
    }
}

fn is_legal_movement(state: &[[char; 3]; 3], position: &Point) -> bool {
    let char_in_position = state[position.row][position.col];
    if char_in_position == P1 || char_in_position == P2 {
        return false;
    }
    return true;
}

fn change_player(current_player: &mut char) {
    if *current_player == P1 {
        *current_player = P2;
    } else {
        *current_player = P1;
    }
}

fn get_next_ia_position(state: &[[char; 3]; 3]) -> Point {
    for r in 0..state.len() {
        for c in 0..state[r].len() {
            if state[r][c] == '⬜' {
                return Point { row: r, col: c };
            }
        }
    }
    return Point { row: 0, col: 0 };
}

// ----------------------------------------------------------------------------
fn game_loop(current_player: &mut char, state: &mut [[char; 3]; 3], ia_num: &i8) {
    while get_winner(&state) == WinState::CONTINUE {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        print_board(&state);
        let mut position;
        if (*ia_num == 1 && *current_player == P2) || *ia_num == 2 {
            thread::sleep(ONE_SEC);
            position = get_next_ia_position(&state);
        } else {
            println!("\nNext position {}", current_player);
            position = get_next_position();
            while !is_legal_movement(&state, &position) {
                eprintln!("Not allowed!");
                position = get_next_position();
            }
        }
        state[position.row][position.col] = *current_player;
        // println!("{} to row: {}, col: {}", current_player, position.row + 1, position.col + 1);
        change_player(current_player);
    }

    // end
    print!("\x1B[2J\x1B[1;1H"); // clear screen
    print_board(&state);
    if get_winner(&state) == WinState::PLAYER1 {
        println!("\nGAME OVER\n{} WINS\n", P1);
    } else if get_winner(&state) == WinState::PLAYER2 {
        println!("\nGAME OVER\n{} WINS\n", P2);
    } else {
        println!("\nGAME OVER\nNOBODY WINS\n");
    }
}

fn main() {
    // setup
    let mut rng = rand::thread_rng();
    let mut state = [['⬜'; 3]; 3];
    let ia_num: i8 = 2;
    let mut current_player = P1;
    set_first_player(&mut rng, &mut current_player);

    // game
    game_loop(&mut current_player, &mut state, &ia_num);
}
