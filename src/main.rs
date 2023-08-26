use rand::Rng;
use raylib::prelude::*;

const MAP_SIZE: i32 = 100;
const SCALING: i32 = 3;
const RANDOM_PROBABILITY: u8 = 5;

struct Map {
    lower: Vec<Vec<bool>>,
    upper: Vec<Vec<bool>>,
    switcher: bool,
}

impl Map {
    fn new(size: i32) -> Self {
        Map {
            lower: vec![vec![false; size as usize]; size as usize],
            upper: vec![vec![false; size as usize]; size as usize],
            switcher: false,
        }
    }

    fn randomize(mut self) -> Self {
        // Use current map determined by 'switcher'
        let mut current = &mut self.lower;
        if self.switcher == true {
            current = &mut self.upper;
        }
        // Randomize
        let mut rng = rand::thread_rng();
        for i in 0..MAP_SIZE as usize {
            for j in 0..MAP_SIZE as usize {
                let r = rng.gen_range(0..RANDOM_PROBABILITY);
                if r == 0 {
                    current[i][j] = true;
                }
            }
        }
        self
    }

    fn read(&self, i: usize, j: usize) -> bool {
        match self.switcher {
            false => self.lower[i][j], // Lower
            true => self.upper[i][j],  // Upper
        }
    }

    // fn set_next(&mut self, i: usize, j: usize, value: bool) {
    //     match self.switcher {
    //         false => self.upper[i][j] = value,
    //         true => self.lower[i][j] = value,
    //     }
    // }

    // fn next_generation(&mut self) {}
}

fn main() {
    // Initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(MAP_SIZE * SCALING, MAP_SIZE * SCALING)
        .title("Game of life")
        .build();

    // Initialize map
    let map = Map::new(MAP_SIZE).randomize();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Set background
        d.clear_background(Color::BLACK);

        // Draw map
        for i in 0..MAP_SIZE as usize {
            for j in 0..MAP_SIZE as usize {
                if map.read(i, j) == true {
                    // d.draw_pixel(i as i32 * SCALING, j as i32 * SCALING, Color::WHITE);
                    d.draw_rectangle(
                        i as i32 * SCALING,
                        j as i32 * SCALING,
                        SCALING,
                        SCALING,
                        Color::WHITE,
                    );
                }
            }
        }
    }
}
