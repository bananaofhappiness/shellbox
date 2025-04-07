use std::{io::{self, stdout, Write}, thread::sleep, time::Duration};
use crossterm::{
    cursor::{Hide, MoveTo}, event::poll, execute, queue, style::Print, terminal::{size, Clear, ClearType}
};
use rand::{self, random_bool};

struct GameWorld {
    width: usize,
    height: usize,
    previous: Vec<u8>,
    current: Vec<u8>,
}

impl GameWorld {
    fn new() -> io::Result<Self> {
        let (width, height) = size()?;
        let (width, height) = (width as usize, height as usize);
        Ok(Self {
            width,
            height,
            previous: vec![0; width * height], 
            current: vec![0; width * height],
        })
    }

    fn convert_to_char(x: u8) -> char {
        if x == 0 {
            return ' ';
        }
        '*'
    }

    fn render(&mut self) -> io::Result<()> {
        for x in 0..self.width*self.height {
            let x_pos = (x % self.width) as u16;
            let y_pos = (x / self.width) as u16;
            queue!(stdout(), MoveTo(x_pos, y_pos), Print(Self::convert_to_char(self.current[x])))?;
        }
        stdout().flush()?;
        self.previous = self.current.clone();
        Ok(())
    }

    fn get_previous_cell_below(&mut self, x: usize) -> Option<u8> {
        if (x / self.width) == self.height - 1 {
            return None;
        }
        Some(self.current[x + self.width])
    }

    fn get_previous_diagonal_cell_below(&mut self, x: usize) -> (Option<u8>, Option<u8>) {
        if (x / self.width) == self.height - 1 {
            return (None, None);
        }
        
        if (x % self.width) == self.width - 1 {
            return (Some(self.current[x + self.width - 1]), None);
        }

        if (x % self.width) == 0 {
            return (None, Some(self.current[x + self.width + 1]));
        }

        (Some(self.previous[x + self.width - 1]), Some(self.previous[x + self.width + 1]))
    }

    fn fall_down(&mut self, x: usize) {
        self.current[x] = 0;
        self.current[x + self.width] = 1;
    }

    fn fall_left(&mut self, x: usize) {
        self.current[x] = 0;
        self.current[x + self.width - 1] = 1;
    }

    fn fall_right(&mut self, x: usize) {
        self.current[x] = 0;
        self.current[x + self.width + 1] = 1;
    }

    fn update(&mut self) {
        for x in (0..self.width*self.height).rev() {
            if self.previous[x] == 1 {
                if self.get_previous_cell_below(x).is_some_and(|x| x == 0) {
                    self.fall_down(x);
                }
                else {
                    let (left, right) = self.get_previous_diagonal_cell_below(x);

                    if left.is_some_and(|x| x == 0) && right.is_some_and(|x| x == 0) {
                        if random_bool(0.5) {
                            self.fall_left(x);
                        }
                        else {
                            self.fall_right(x);
                        }
                    }

                    if left.is_some_and(|x| x == 0) && (right.is_none() || right.is_some_and(|x| x == 1)) {
                        self.fall_left(x);
                    }

                    if right.is_some_and(|x| x == 0) && (left.is_none() || left.is_some_and(|x| x == 1)) {
                        self.fall_right(x);
                    }
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    println!();
    let mut game_world = GameWorld::new()?;
    game_world.render()?;
    loop {
        // game_world.current[rand::random_range(0..game_world.width)] = 1;
        game_world.current[game_world.width / 2] = 1;
        game_world.current[game_world.width / 3] = 1;
        game_world.current[game_world.width / 3 * 2] = 1;
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