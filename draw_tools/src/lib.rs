use ncurses::*;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub c: Vec<char>,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub fn new() -> Self {
        Object {
            y: 0,
            x: get_col(),
            c: vec![get_real_char()],
        }
    }
}

pub fn debug_print(s: &str) {
    initscr();
    start_color();
    init_pair(1, COLOR_BLACK, COLOR_WHITE);
    attron(A_BOLD() | COLOR_PAIR(1));
    addstr(s);
    refresh();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

pub fn get_max_x() -> i32 {
    COLS()
}

pub fn get_max_y() -> i32 {
    LINES()
}

pub fn draw_cells(data: &Vec<Object>) -> Result<(), String> {
    initscr();
    start_color();
    
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, 10, COLOR_BLACK);
    init_pair(3, 46, COLOR_BLACK);
    init_pair(4, 40, COLOR_BLACK);
    init_pair(5, 34, COLOR_BLACK);
    init_pair(6, 28, COLOR_BLACK);
    init_pair(7, 22, COLOR_BLACK);

    for d in data {
        for (n, c) in d.c.iter().enumerate() {
            if n == 0 {
                attron(A_ALTCHARSET() | A_BOLD() | COLOR_PAIR(1));
            } else if n > 0 && n <= 4 {
                attron(A_ALTCHARSET() | COLOR_PAIR(2));
            } else if n > 5 && n <= 8 {
                attron(A_ALTCHARSET() | COLOR_PAIR(3));
            } else if n > 9 && n <= 12 {
                attron(A_ALTCHARSET() | COLOR_PAIR(4));
            } else if n > 13 && n <= 16 {
                attron(A_ALTCHARSET() | COLOR_PAIR(5));
            } else if n > 17 && n <= 20 {
                attron(A_ALTCHARSET() | COLOR_PAIR(6));
            } else if n > 21 && n <= 24 {
                attron(A_ALTCHARSET() | COLOR_PAIR(7));
            }

            let ch = *c as chtype;
            mvaddch((d.y) - (n as i32), d.x, ch);
        }
    }

    refresh();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    Ok(())
}

pub fn clear_screen() {
    ncurses::erase();
}

pub fn get_real_char() -> char {
    let mut rng = rand::thread_rng();
    let mut random_char = rng.gen::<char>();
    loop {
        if random_char.is_ascii_graphic() {
            return random_char;
        } else {
            random_char = rng.gen::<char>();
        }
    }
}

fn get_col() -> i32 {
    let mut rng = rand::thread_rng();
    let max_y = COLS();
    rng.gen_range(0..max_y)
}
