extern crate ncurses;

use ncurses::*;
//use rand::Rng;
use snake::game_objects::*;

fn prep_screen() {

    initscr();
    clear();
    noecho();
    cbreak();
    timeout(150);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    keypad(stdscr(), true);

}

fn end(message: &str) {

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

fn print_borders() {

    for i in 2 .. (COLS() - 1) {

        mvprintw(2, i, "X");
        mvprintw(LINES() - 2, i, "X");

    }

    for i in 2 .. (LINES() - 1) {

        mvprintw(i, 2, "X");
        mvprintw(i, COLS() - 2, "X");

    }

}

fn move_snake(mut snakey: Snake) -> Snake {

    for i in (1usize .. snakey.size).rev() {
        
        snakey.coords[i].ycoord = snakey.coords[i - 1].ycoord;
        snakey.coords[i].xcoord = snakey.coords[i - 1].xcoord;

    }

    snakey.coords[0].xcoord += snakey.xvel;
    snakey.coords[0].ycoord += snakey.yvel;

    return snakey;

}

fn test_collision(snakey: &Snake) {

    test_self_collision(snakey);
    test_lower_bounds_collision(snakey);
    test_upper_bounds_collision(snakey);

}

fn test_self_collision(snakey: &Snake) {

    for i in 1usize .. snakey.size {

        if snakey.coords[i].ycoord == snakey.coords[0].ycoord && snakey.coords[i].xcoord == snakey.coords[0].xcoord {

            if snakey.xvel != 0 || snakey.yvel != 0 {

                end("Game Over! Press any key to continue");

            }

        }

    }

}

fn test_lower_bounds_collision(snakey: &Snake) {

    if snakey.coords[0].ycoord < 3 || snakey.coords[0].xcoord < 3 {

        end("Game Over! Press any key to continue");

    }

}

fn test_upper_bounds_collision(snakey: &Snake) {

    if snakey.coords[0].ycoord > (LINES() - 3) || snakey.coords[0].xcoord > (COLS() - 3) {

        end("Game Over! Press any key to continue");

    }

}

fn main() {

    prep_screen();

    let mut snakey: Snake = Snake::new((COLS() - 3) as usize, (LINES() - 3) as usize);
    let mut pellet: Coords = Coords::new(0, 0);

    pellet.rand_coords(COLS(), LINES());

    snakey.size = 1;
    snakey.coords[0].xcoord = 3;
    snakey.coords[0].ycoord = 3;


    let mut ch: i32 = getch();

    while ch != 113 { //113 is the ascii representation of q, I can't use 'q' b/c Rust doesn't use ascii but rather uses unicode.

        snakey = get_direction(snakey, ch);

        clear();

        mvprintw(0,0, "Press 'q' to quit.");

        print_borders();

        let score_stri: String = format!("-Score: {}-", snakey.size);

        mvprintw(2,3, &score_stri);

        snakey = move_snake(snakey);

        test_collision(&snakey);

        if snakey.coords[0].ycoord == pellet.ycoord && snakey.coords[0].xcoord == pellet.xcoord {

            if snakey.size < ((COLS() - 3) * (LINES() - 3)) as usize {

                snakey.size += 1;

            } else {

                end("Congrats, You Win!");
            }

            pellet.rand_coords(COLS() - 3, LINES() - 3);

        }

        for i in 0 .. snakey.coords.iter().count() {

            //let coord_stri: String = format!("x: {}, y: {}", snakey.coords[i].xcoord, snakey.coords[i].ycoord);
            //mvprintw((i+2) as i32, 1, &coord_stri);

            mvprintw(snakey.coords[i].ycoord, snakey.coords[i].xcoord, "*");

        }

        //let coord_stri: String = format!("x: {}, y: {}", pellet.xcoord, pellet.ycoord); 
        //mvprintw(1,1, &coord_stri);
        
        mvprintw(pellet.ycoord, pellet.xcoord, "*");

        ch = getch();

    }


    //mvprintw(1,1, &set_stri);

    refresh();

    end("Thanks for playing");



}
