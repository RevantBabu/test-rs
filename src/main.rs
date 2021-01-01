extern crate termion;

mod table;

use termion::{raw::IntoRawMode, screen::AlternateScreen};
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    //let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdout = AlternateScreen::from(stdout()).into_raw_mode().unwrap();

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}\n\r",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(1, 1),
           // Hide the cursor.
           termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    //let (w, h) = termion::terminal_size().unwrap();
    let origin = table::Point {
        x:5,
        y:5
    };
    let mut table = table::Table {
        origin: origin,
        width: 150,
        height: 10,
        offset_row: 0,
        active_row: 0,
        rows: vec![
            vec![String::from("Number"), String::from("NAME"), String::from("CLUSTER"), String::from("NAMESPACE")],
            vec![String::from("1"), String::from("entity1"), String::from("entity2"), String::from("entity3")],
            vec![String::from("2"), String::from("entity4"), String::from("entity5"), String::from("entity6")],
            vec![String::from("3"), String::from("entity7"), String::from("entity8"), String::from("entity9")],
        ],
        stdout: &mut stdout,
    };

    table.draw();


    for c in stdin.keys() {
        // Clear the current line.
        //write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(c)   => println!("{}", c as u32),
            Key::Alt(c)    => println!("Alt-{}", c),
            Key::Ctrl(c)   => println!("Ctrl-{}", c),
            Key::Left      => println!("<left>"),
            Key::Right     => println!("<right>"),
            Key::Up        => table.scroll_up(),
            Key::Down      => table.scroll_down(),
            _              => println!("Other"),
        }

        // Flush again.
        //stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
