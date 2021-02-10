use rand::Rng;

#[derive(Clone, Copy)]
pub struct Coords {

    pub xcoord: i32,
    pub ycoord: i32,

}


impl Coords {

    pub fn new(xcoord: i32, ycoord: i32) -> Coords {

        Coords { xcoord: xcoord, ycoord: ycoord }

    }

    pub fn rand_coords(&mut self, x: i32, y: i32) {

        let mut rng = rand::thread_rng();

        self.xcoord = rng.gen_range(3 .. x);
        self.ycoord = rng.gen_range(3 .. y);

    }

}


pub struct Snake {

    pub xvel: i32,
    pub yvel: i32,

    pub size: usize,

    pub coords: Vec<Coords>,

}

impl Snake {

    pub fn new() -> Snake {

        let tmp_snake: Snake = Snake {xvel: 0, yvel: 0, size: 0, coords: vec![Coords::new(0,0); 200]};

        return tmp_snake;

    }

}
