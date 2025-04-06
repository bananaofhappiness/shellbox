use std::{io::{self, stdout, Write}, thread::sleep, time::Duration};
use crossterm::{
    cursor::{Hide, MoveTo}, event::poll, execute, queue, style::Print, terminal::{size, Clear, ClearType}
};
use rand;

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
        '#'
    }

    fn render(&mut self) -> io::Result<()> {
        for x in 0..self.current.len() {
            let x_pos = (x % self.width) as u16;
            let y_pos = (x / self.width) as u16;
            queue!(stdout(), MoveTo(x_pos, y_pos), Print(Self::convert_to_char(self.current[x])))?;
        }
        stdout().flush()?;
        self.previous = self.current.clone();
        Ok(())
    }

    fn get_current_cell_below(&mut self, x: usize) -> Option<&mut u8> {
        if (x / self.width) == self.height - 1 {
            return None;
        }
        Some(&mut self.current[x + self.width])
    }

    fn get_previous_cell_below(&mut self, x: usize) -> Option<&mut u8> {
        if (x / self.width) == self.height - 1 {
            return None;
        }
        Some(&mut self.current[x + self.width])
    }

    fn update(&mut self) {
        for x in 0..self.current.len() {
            if self.previous[x] == 1 {
                if let Some(prev_cell) = self.get_previous_cell_below(x) {
                    if *prev_cell == 0 {
                        self.current[x] = 0;
                        *self.get_current_cell_below(x).unwrap() = 1;
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
        game_world.current[rand::random_range(0..game_world.width)] = 1;
        game_world.render()?;
        game_world.update();
        sleep(Duration::from_millis(100));
    }
    Ok(())
}