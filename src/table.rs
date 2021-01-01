use std::io::{Write};
use termion::{color, style};

pub struct Point {
    pub x: u16,
    pub y: u16,
}

pub struct Table<'a> {
    pub origin: Point,
    pub width: u16,
    pub height: u16,
    pub active_row: u16,
    pub offset_row: u16,
    pub rows: Vec<Vec<String>>,
    pub stdout: &'a mut termion::raw::RawTerminal<termion::screen::AlternateScreen<std::io::Stdout>>,
}

impl<'a> Table<'a> {

    pub fn draw(&mut self) {
        let x = self.origin.x;
        let y = self.origin.y;
        let height = self.height;
        let width = self.width;

        write!(self.stdout, "{}", style::Italic).unwrap();
        self.stdout.flush().unwrap();

        for y1 in y..y+height {
            let mut s = String::from("");
            for x1 in x..x+width {
                if y1==y && x1==x {s.push_str("┌");}
                else if y1==y && x1==x+width-1 {s.push_str("┐");}
                else if y1==y+height-1 && x1==x {s.push_str("└");}
                else if y1==y+height-1 && x1==x+width-1 {s.push_str("┘");}
                else if y1==y || y1==y+height-1 {s.push_str("─");}
                else if x1==x || x1==x+width-1 {s.push_str("│");}
                else {s.push_str(" ");}
            }
            write!(self.stdout, "{}{}", termion::cursor::Goto(x, y1), s).unwrap();
            self.stdout.flush().unwrap();
        }

        let mut counter_y = 0;
        let ln = std::cmp::min(self.height as usize-3+self.offset_row as usize, self.rows.len()-1);
        for i in &self.rows[self.offset_row as usize..=ln] {
            let mut counter_x = 0;
            if counter_y+self.offset_row==self.active_row+1 {
                let s = " ".repeat(width as usize-2);
                write!(self.stdout,
                    "{}{}{}{}",
                    termion::cursor::Goto(x+1, y+1+counter_y),
                    color::Bg(color::White),
                    color::Fg(color::Black),
                    s).unwrap();
            }
            for j in i {
                write!(self.stdout, "{}{}", termion::cursor::Goto(x+1+counter_x*40, y+1+counter_y), j).unwrap();
                counter_x+=1;
            }
            if counter_y+self.offset_row==self.active_row+1 {
                write!(self.stdout,
                    "{}{}",
                    color::Bg(color::Reset),
                    color::Fg(color::Reset)).unwrap();
            }

            counter_y+=1;
        }
        self.stdout.flush().unwrap();
    }

    pub fn scroll_down(&mut self) {
        if self.active_row<self.rows.len() as u16-2 {
            self.active_row += 1;
            if self.active_row>=self.height-3 {
                self.offset_row+=1;
            }
            self.draw();
        }
    }

    pub fn scroll_up(&mut self) {
        if self.active_row>0 {
            self.active_row -= 1;
            if self.active_row<=3 && self.offset_row>0 {
                self.offset_row-=1;
            }
            self.draw();
        }
    }
}