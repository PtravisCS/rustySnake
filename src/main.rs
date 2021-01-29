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

fn prep_screen() {

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

fn get_direction(mut snakey: Snake, ch: i32) -> Snake {

    if ch == KEY_UP {

        snakey.xvel = 0;
        snakey.yvel = -1;

    } else if ch == KEY_DOWN {

        snakey.xvel = 0;
        snakey.yvel = 1;

    } else if ch == KEY_LEFT {

        snakey.xvel = -1;
        snakey.yvel = 0;

    } else if ch == KEY_RIGHT {

        snakey.xvel = 1;
        snakey.yvel = 0;

    }
    
    return snakey;

}

fn main() {

    //let set_stri: String = format!("x: {}, y: {}", set.xcoord, set.ycoord);

    let mut snakey: Snake = Snake::new();

    prep_screen();

    let mut ch: i32 = getch();

    while ch != 113 { //113 is the ascii representation of q, I can't use 'q' b/c Rust doesn't use ascii but rather uses unicode.

        snakey = get_direction(snakey, ch);

        clear();

        mvprintw(0,0, "Press 'q' to quit.");


        for i in 0 .. snakey.coords.iter().count() {

            let coord_stri: String = format!("x: {}, y: {}", snakey.coords[i].xcoord, snakey.coords[i].ycoord);
            mvprintw((i+2) as i32, 1, &coord_stri);

        }

        ch = getch();

    }


    //mvprintw(1,1, &set_stri);

    refresh();

    end("Thanks for playing".to_string());



}
