use std::{collections::HashSet, io::{self, stdout, ErrorKind, Write}, thread::sleep, time::Duration};
use crossterm::{
    cursor::{Hide, MoveTo}, event::poll, execute, queue, style::Print, terminal::{size, Clear, ClearType}
};
use rand::{self, random_bool};

const BRAILE_COLS: usize = 2;
const BRAILE_ROWS: usize = 4;
const BRAILE_MASK: u32 = 0x2800;


struct GameWorld {
    screen_width: usize,
    screen_height: usize,
    world_width: usize,
    world_height: usize,
    particles: Vec<bool>,
    previous_particles: Vec<bool>,
}

impl GameWorld {
    fn new() -> io::Result<Self> {
        let (width, height) = size()?;
        let (screen_width, screen_height) = (width as usize, height as usize);
        let (world_width, world_height) = (screen_width * BRAILE_COLS, screen_height * BRAILE_ROWS);
        Ok(Self {
            screen_width,
            screen_height,
            world_width,
            world_height,
            particles: vec![false; world_width * world_height],
            previous_particles: vec![false; world_width * world_height],
        })
    }

    fn convert_to_char(x: u8) -> io::Result<char> {
        let ch = char::from_u32(x as u32 | BRAILE_MASK);
        match ch {
            Some(ch) => return Ok(ch),
            None => return Err(io::Error::new(ErrorKind::InvalidData, "Couldn't convert bits to char")),
        }
    }

    // TODO: render bug: the down-right symbol does not render
    fn render(&mut self) -> io::Result<()> {
        let mut frame = Vec::new();
        let mut i = 0;
        while i < self.world_height * self.world_width {
            
            let mut sum: u8 = 0;
            let mut coef = 1;
            
            for j in 0..2 {
                for k in 0..3 {
                    if self.particles[i + j + self.world_width * k] {
                        sum += 1 * coef;
                    }
                    coef *= 2;
                }
                coef = 8;
            }
            
            if self.particles[i + self.world_width * 3] {
                sum += 64;
            }
            if self.particles[i + 1 + self.world_width * 3] {
                sum += 128;
            }
            
            frame.push(sum);
            i += 2;
            
            if (i % self.world_width) == self.world_width - 2 {
                i += self.world_width * 3;
            }
            
            if i >= self.world_width * (self.world_height - 3) - 1 {
                break;
            }
        }
        
        for (i, x) in frame.iter().enumerate() {
            let x_pos = i as u16 % self.screen_width as u16;
            let y_pos = i as u16 / self.screen_width as u16;
            queue!(stdout(), MoveTo(x_pos, y_pos), Print(Self::convert_to_char(*x)?))?;
        }
        stdout().flush()?;
        Ok(())
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #
        // # # # # # # # # # # # #

        // 1 8
        // 2 16
        // 4 32
        // 64 128
    }

    fn get_cell_below(&mut self, x: usize) -> Option<bool> {
        if (x / self.world_width) == self.world_height - 1 {
            return None;
        }
        Some(self.particles[x + self.world_width])
    }

    fn get_diagonal_cells_below(&mut self, x: usize) -> (Option<bool>, Option<bool>) {
        if (x / self.world_width) == self.world_height - 1 {
            return (None, None);
        }
        
        if (x % self.world_width) == self.world_width - 1 {
            return (Some(self.particles[x + self.world_width - 1]), None);
        }

        if (x % self.world_width) == 0 {
            return (None, Some(self.particles[x + self.world_width + 1]));
        }

        (Some(self.particles[x + self.world_width - 1]), Some(self.particles[x + self.world_width + 1]))
    }

    fn fall_down(&mut self, x: usize) {
        self.particles[x] = false;
        self.particles[x + self.world_width] = true;
    }

    fn fall_left(&mut self, x: usize) {
        self.particles[x] = false;
        self.particles[x + self.world_width - 1] = true;
    }

    fn fall_right(&mut self, x: usize) {
        self.particles[x] = false;
        self.particles[x + self.world_width + 1] = true;
    }


    fn update(&mut self) {
        for x in (0..self.world_width * self.world_height).rev() {
            if !self.particles[x] {
                continue;
            }
            if self.get_cell_below(x).is_some_and(|x| !x) {
                self.fall_down(x);
            }
            else {
                let (left, right) = self.get_diagonal_cells_below(x);

                if left.is_some_and(|x| !x) && right.is_some_and(|x| !x) {
                    if random_bool(0.5) {
                        self.fall_left(x);
                    }
                    else {
                        self.fall_right(x);
                    }
                }

                if left.is_some_and(|x| !x) && (right.is_none() || right.is_some_and(|x| x)) {
                    self.fall_left(x);
                }

                if right.is_some_and(|x| !x) && (left.is_none() || left.is_some_and(|x| x)) {
                    self.fall_right(x);
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    println!();
    let mut game_world = GameWorld::new()?;
    // game_world.render()?;
    // game_world.particles[0] = true;
    // game_world.particles[1] = true;
    // game_world.particles[2] = true;
    // game_world.particles[3] = true;
    // game_world.particles[33] = true;
    // game_world.particles[2270] = true;
    // game_world.particles[31] = true;
    loop {
        game_world.particles[rand::random_range(0..game_world.world_width)] = true;
        // game_world.particles[game_world.world_height / 2] = true;
        // game_world.particles[game_world.world_height / 3] = true;
        // game_world.particles[game_world.world_height / 3 * 2] = true;
        game_world.render()?;
        game_world.update();
        sleep(Duration::from_millis(100));
    }

    // game_world.current[game_world.width * game_world.height - 10] = 1;
    // game_world.current[game_world.width * game_world.height - 11] = 1;
    // game_world.current[game_world.width * game_world.height - 12] = 1;
    // game_world.current[game_world.width * (game_world.height - 1) - 10] = 1;
    // game_world.current[game_world.width * (game_world.height - 1) - 12] = 1;
    // game_world.current[game_world.width * (game_world.height - 2) - 10] = 1;
    // game_world.current[game_world.width * (game_world.height - 2) - 12] = 1;
    // game_world.current[game_world.width * (game_world.height - 3) - 10] = 1;
    // game_world.current[game_world.width * (game_world.height - 3) - 12] = 1;
    // game_world.render()?;
    // sleep(Duration::from_millis(100));
    // game_world.update();
    // game_world.render()?;
    // game_world.update();
    // game_world.render()?;
    // game_world.update();
    // game_world.render()?;
    // game_world.update();
    // game_world.render()?;
    Ok(())
}