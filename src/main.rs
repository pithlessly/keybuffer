use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::fs::OpenOptions;
use std::io::Write as _;

fn main() {
    let raw_mode = termion::get_tty().unwrap().into_raw_mode().unwrap();
    let stdin = raw_mode.try_clone().unwrap();

    let file_name = std::env::args().nth(1).unwrap();
    let mut out_file = OpenOptions::new().append(true).open(file_name).unwrap();

    macro_rules! o {
        ($e0:expr, $e1:expr) => {
            write!(out_file, "{}{}\r\n", $e0, $e1).unwrap();
        };
        ($e:expr) => {
            write!(out_file, "{}\r\n", $e).unwrap();
        }
    }

    for key in stdin.keys() {
        if let Ok(key) = key {
            match key {
                Key::Ctrl('c') | Key::Ctrl('d') => break,

                Key::Backspace  => o!("backspace"),
                Key::Left       => o!("left"),
                Key::Right      => o!("right"),
                Key::Up         => o!("up"),
                Key::Down       => o!("down"),
                Key::Home       => o!("home"),
                Key::End        => o!("end"),
                Key::PageUp     => o!("page up"),
                Key::PageDown   => o!("page down"),
                Key::Char('\n') => o!("enter"),
                Key::Char('\t') => o!("tab"),
                Key::BackTab    => o!("back tab"),
                Key::Delete     => o!("delete"),
                Key::Insert     => o!("insert"),
                Key::F(i)       => o!("f", i),
                Key::Char(c)    => o!(c),
                Key::Alt(c)     => o!("alt ", c),
                Key::Ctrl(c)    => o!("ctrl ", c),
                Key::Null       => o!("null"),
                Key::Esc        => o!("esc"),

                _ => {}
            }
        }
    }
}
