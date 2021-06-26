use std::io;
use rand::Rng;
use rand::rngs::ThreadRng;

// ----------------------------------------------------------------------------
struct Point {
    row: usize,
    col: usize
}

static P1: char = '⚫';
static P2: char = '🔴';

// ----------------------------------------------------------------------------
fn set_first_player(rng: &mut ThreadRng, current_player: &mut char) {
    if rng.gen_range(0..2) == 0 { // (0..2) between 0 and 1
        *current_player = P1;
    } else {
        *current_player = P2;
    }
}

fn is_game_finished(state: &[[char; 3]; 3]) -> bool {
    // let mut finished = false;
    // rows
    for row in state {
        let row_str: String = row.iter().collect();
        // println!("row {:?} -> '{}'", row, row_str);
        if row_str == "⚫⚫⚫" || row_str == "🔴🔴🔴" {
            return true;
        }
    }
    // cols
    for col_num in 0..3 {
        let col_str: String = format!("{}{}{}", state[0][col_num], state[1][col_num], state[2][col_num]);
        // println!("col '{}'", col_str);
        if col_str == "⚫⚫⚫" || col_str == "🔴🔴🔴" {
            return true;
        }
    }
    // diagonals
    let left_diagonal_str = format!("{}{}{}", state[0][0], state[1][1], state[2][2]);
    // println!("{}", left_diagonal_str);
    if left_diagonal_str == "⚫⚫⚫" || left_diagonal_str == "🔴🔴🔴" {
        return true;
    }
    let right_diagonal_str = format!("{}{}{}", state[0][2], state[1][1], state[2][0]);
    // println!("{}", right_diagonal_str);
    if right_diagonal_str == "⚫⚫⚫" || right_diagonal_str == "🔴🔴🔴" {
        return true;
    }
    return false;
}

/*
fn get_winner(state: &[[char; 3]; 3]) -> i8 {
    // rows
    for row in state {
        let row_str: String = row.iter().collect();
        if row_str == "⚫⚫⚫" {
            return true;
        } else if row_str == "🔴🔴🔴" {

        }
    }
    // cols
    for col_num in 0..3 {
        let col_str: String = format!("{}{}{}", state[0][col_num], state[1][col_num], state[2][col_num]);
        // println!("col '{}'", col_str);
        if col_str == "⚫⚫⚫" || col_str == "🔴🔴🔴" {
            return true;
        }
    }
    // diagonals
    let left_diagonal_str = format!("{}{}{}", state[0][0], state[1][1], state[2][2]);
    // println!("{}", left_diagonal_str);
    if left_diagonal_str == "⚫⚫⚫" || left_diagonal_str == "🔴🔴🔴" {
        return true;
    }
    let right_diagonal_str = format!("{}{}{}", state[0][2], state[1][1], state[2][0]);
    // println!("{}", right_diagonal_str);
    if right_diagonal_str == "⚫⚫⚫" || right_diagonal_str == "🔴🔴🔴" {
        return true;
    }
}
*/

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

// ----------------------------------------------------------------------------
fn game_loop(current_player: &mut char, state: &mut [[char; 3]; 3]) {
    let mut limit: i16 = 9;
    while !is_game_finished(&state) {
        print!("\x1B[2J\x1B[1;1H"); // clear screen
        print_board(&state);
        println!("\nNext position {}", current_player);
        let mut position = get_next_position();
        while !is_legal_movement(&state, &position) {
            eprintln!("Not allowed!");
            position = get_next_position();
        }
        state[position.row][position.col] = *current_player;
        println!("{} to row: {}, col: {}", current_player, position.row + 1, position.col + 1);
        change_player(current_player);
        limit -= 1;
        if limit < 1 {
            break;
        }
    }

    // end
    print!("\x1B[2J\x1B[1;1H"); // clear screen
    print_board(&state);
    if limit >= 0 && is_game_finished(&state) {
        change_player(current_player);
        println!("\nGAME OVER\n{} WINS\n", current_player);
    } else {
        println!("\nGAME OVER\nNOBODY WINS\n");
    }
}

fn main() {
    // setup
    let mut rng = rand::thread_rng();
    let mut state = [['⬜'; 3]; 3];
    let mut current_player = P1;
    set_first_player(&mut rng, &mut current_player);

    // game
    game_loop(&mut current_player, &mut state);
}
