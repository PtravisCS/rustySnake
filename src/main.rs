extern crate ncurses;

use ncurses::*;

#[derive(Clone)]
struct Coords {

    xcoord: i32,
    ycoord: i32,

}


impl Coords {

    fn new(xcoord: i32, ycoord: i32) -> Coords {

        Coords { xcoord: xcoord, ycoord: ycoord }

    }

}

struct Snake {

    xvel: i32,
    yvel: i32,

    size: i32,

    coords: Vec<Coords>,

}

impl Snake {

    fn new() -> Snake {

        let tmp_snake: Snake = Snake {xvel: 0, yvel: 0, size: 0, coords: vec![Coords::new(0,0); 200]};

        return tmp_snake;

    }

}

fn prepScreen() {

    initscr();
    clear();
    noecho();
    cbreak();
    timeout(150);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

}

fn end(message: String) {

    mvprintw((LINES() - 1) / 2, (COLS() - (message.chars().count() as i32)) / 2, &message);
    
    timeout(-1);

    getch();

    endwin();

    std::process::exit(0);

}

fn main() {

    //let set_stri: String = format!("x: {}, y: {}", set.xcoord, set.ycoord);

    let snakey: Snake = Snake::new();

    prepScreen();

    let mut ch: i32 = getch();

    while ch != 113 {

        ch = getch();
        

    }

    for i in 0 .. snakey.coords.iter().count() {

        let coord_stri: String = format!("x: {}, y: {}", snakey.coords[i].xcoord, snakey.coords[i].ycoord);
        mvprintw((i+2) as i32, 1, &coord_stri);


    }

    //mvprintw(1,1, &set_stri);

    refresh();

    end("Thanks for playing".to_string());



}
