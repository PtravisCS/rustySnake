extern crate termion;

use termion::*;
use termion::raw::*;
use termion::event::Key;
use termion::input::TermRead;
use snake::game_objects::*;
use std::io::Write;
use std::{thread, time};

fn get_cols() -> u16 {

    let dimensions_tuple: (u16, u16) = terminal_size().unwrap();

    let cols: u16 = dimensions_tuple.1;

    return cols;
}

fn get_rows() -> u16 {

    let dimensions_tuple: (u16, u16) = terminal_size().unwrap();

    let rows: u16 = dimensions_tuple.0;

    return rows;

}

fn refresh_scrn() {

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    stdout.flush().unwrap();

}

fn clr_scrn() {

    print!("{}", clear::All);

}

fn up_key() -> event::Key {

    return event::Key::Up;

}

fn down_key() -> event::Key {

    return event::Key::Down;

}

fn left_key() -> event::Key {

    return event::Key::Left;

}

fn right_key() -> event::Key {

    return event::Key::Right;

}

fn no_echo() {

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", cursor::Hide);

}

fn prep_screen() {

    clr_scrn();
    no_echo();

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1,1), cursor::Hide);

}

fn end(message: &str) {

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{goto} {str}", goto = cursor::Goto((get_rows() - 1u16) / 2u16, (get_cols() - (message.chars().count() as u16)) / 2u16), str = &message).unwrap();
    
    std::process::exit(0);

}

fn get_direction(mut snakey: Snake, ch: event::Key) -> Snake {

    if ch == up_key() {

        snakey.xvel = 0;
        snakey.yvel = -1;

    } else if ch == down_key() {

        snakey.xvel = 0;
        snakey.yvel = 1;

    } else if ch == left_key() {

        snakey.xvel = -1;
        snakey.yvel = 0;

    } else if ch == right_key() {

        snakey.xvel = 1;
        snakey.yvel = 0;

    }
    
    return snakey;

}

fn print_borders() {

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    for i in 2u16 .. (get_cols() - 1u16) {

        write!(stdout, "{goto}X", goto = cursor::Goto(2, i));

    }

    for i in 2u16 .. (get_rows() - 1u16) {

        write!(stdout, "{goto}X", goto = cursor::Goto(i, 2));

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

    if snakey.coords[0].ycoord > (get_rows() - 3u16) as i32 || snakey.coords[0].xcoord > (get_cols() - 3u16) as i32 {

        end("Game Over! Press any key to continue");

    }

}



fn main() {

    prep_screen();

    let mut snakey: Snake = Snake::new((get_cols() - 3u16) as usize, (get_rows() - 3u16) as usize);
    let mut pellet: Coords = Coords::new(0, 0);

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();

    let sleep_duration = time::Duration::from_millis(150);

    pellet.rand_coords(get_cols() as i32, get_rows() as i32);

    snakey.size = 1;
    snakey.coords[0].xcoord = 3;
    snakey.coords[0].ycoord = 3;

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1,1), cursor::Hide);
    refresh_scrn();

    for ch in (stdin.keys()).unwrap() {

        match ch {

            Key::Char('q') => break,
            _              => snakey = get_direction(snakey, ch),

        }

        clr_scrn();

        write!(stdout, "{goto}Press 'q' to quit.", goto = cursor::Goto(0u16,0u16));

        print_borders();

        write!(stdout, "{goto}-Score: {}-", goto = cursor::Goto(2u16,3u16));

        snakey = move_snake(snakey);

        test_collision(&snakey);

        if snakey.coords[0].ycoord == pellet.ycoord && snakey.coords[0].xcoord == pellet.xcoord {

            if snakey.size < ((get_cols() - 3u16) * (get_rows() - 3u16)) as usize {

                snakey.size += 1;

            } else {

                end("Congrats, You Win!");
            }

            pellet.rand_coords((get_cols() - 3u16) as i32, (get_rows() - 3u16) as i32);

        }

        for i in 0 .. snakey.coords.iter().count() {

            //let coord_stri: String = format!("x: {}, y: {}", snakey.coords[i].xcoord, snakey.coords[i].ycoord);
            //mvprintw((i+2) as i32, 1, &coord_stri);

            write!(stdout, "{goto}*", goto = cursor::Goto(snakey.coords[i].ycoord as u16, snakey.coords[i].xcoord as u16));

        }

        //let coord_stri: String = format!("x: {}, y: {}", pellet.xcoord, pellet.ycoord); 
        //mvprintw(1,1, &coord_stri);
        
        write!(stdout, "{goto}*", goto = cursor::Goto(pellet.ycoord as u16, pellet.xcoord as u16));

        thread::sleep(sleep_duration);

    }


    //mvprintw(1,1, &set_stri);

    refresh_scrn();

    end("Thanks for playing");



}
