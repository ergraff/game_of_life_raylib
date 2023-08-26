use rand::Rng;
use raylib::prelude::*;

const MAP_SIZE: i32 = 300;
const SCALING: i32 = 3;
const RANDOM_PROBABILITY: u8 = 5;
const FPS: u32 = 10;

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
        if self.switcher {
            current = &mut self.upper;
        }
        // Randomize
        let mut rng = rand::thread_rng();
        for row in current.iter_mut().take(MAP_SIZE as usize) {
            for val in row {
                let r = rng.gen_range(0..RANDOM_PROBABILITY);
                if r == 0 {
                    *val = true;
                }
            }
        }
        self
    }

    fn alive(&self, i: usize, j: usize) -> bool {
        match self.switcher {
            false => self.lower[i][j],
            true => self.upper[i][j],
        }
    }

    fn set_next(&mut self, i: usize, j: usize, value: bool) {
        match self.switcher {
            false => self.upper[i][j] = value,
            true => self.lower[i][j] = value,
        }
    }

    fn count_neighbors(&self, i: usize, j: usize) -> u8 {
        let mut sum: u8 = 0;
        for ii in [-1, 0, 1].into_iter() {
            for jj in [-1, 0, 1].into_iter() {
                if ii == 0 && jj == 0 {
                    continue;
                }
                let new_i = i as i32 + ii;
                let new_j = j as i32 + jj;
                let bounded = (0..MAP_SIZE).contains(&new_i) && (0..MAP_SIZE).contains(&new_j);
                if bounded && self.alive(new_i as usize, new_j as usize) {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn iterate(&mut self) {
        for i in 0..MAP_SIZE as usize {
            for j in 0..MAP_SIZE as usize {
                let n = self.count_neighbors(i, j);
                let alive = self.alive(i, j);
                match (n, alive) {
                    (3, false) => self.set_next(i, j, true),
                    (2 | 3, true) => self.set_next(i, j, true),
                    _ => self.set_next(i, j, false),
                }
            }
        }
        self.switcher = !self.switcher;
    }
}

fn main() {
    // Initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(MAP_SIZE * SCALING, MAP_SIZE * SCALING)
        .title("Game of life")
        .build();
    rl.set_target_fps(FPS);

    // Initialize map
    let mut map = Map::new(MAP_SIZE).randomize();

    // Begin main game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Set background
        d.clear_background(Color::BLACK);

        // Draw map
        for i in 0..MAP_SIZE as usize {
            for j in 0..MAP_SIZE as usize {
                if map.alive(i, j) {
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

        // Next iteration
        map.iterate();
    }
}
