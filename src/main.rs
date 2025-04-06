use std::{io::{self, stdout, Write}, thread::sleep, time::Duration};
use crossterm::{
    cursor::{Hide, MoveTo}, event::poll, execute, queue, style::Print, terminal::{size, Clear, ClearType}
};

struct FrameBuffer {
    width: usize,
    height: usize,
    previous: Vec<u8>,
    current: Vec<u8>,
}

impl FrameBuffer {
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

    fn render(&mut self) -> io::Result<()> {
        for x in 0..self.current.len() {
            let x_pos = (x % self.width) as u16;
            let y_pos = (x / self.width) as u16;
            queue!(stdout(), MoveTo(x_pos, y_pos), Print(self.current[x]))?;
        }
        stdout().flush()?;
        self.previous = self.current.clone();
        Ok(())
    }
}

fn main() -> io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), Hide)?;
    println!();
    let mut buffer = FrameBuffer::new()?;
    buffer.render()?;
    let mut i = 0;
    while i < buffer.current.len() {
        buffer.current[i] = (i % 10) as u8;
        buffer.render()?;
        i += 1;
    }
    Ok(())
}