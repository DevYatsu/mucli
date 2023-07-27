use std::io::{stdin, stdout, Write};
use std::time::Instant;
use termion::color;
use termion::event::Key;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

pub fn timer_command() {
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}", termion::cursor::BlinkingBar).unwrap();
    write!(
        stdout,
        "{}{}{}Timer!",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        color::Fg(color::Blue)
    )
    .unwrap();

    write!(
        stdout,
        "{}{}Press {}SPACE {}to {}start {}the timer!",
        termion::cursor::Goto(1, 3),
        color::Fg(color::White),
        color::Fg(color::LightYellow),
        color::Fg(color::White),
        color::Fg(color::Red),
        color::Fg(color::White),
    )
    .unwrap();

    stdout.flush().unwrap();

    if let true = break_key_event(' ') {
        write!(
            stdout,
            "{}{}{}Timer!",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            color::Fg(color::Blue)
        )
        .unwrap();

        write!(
            stdout,
            "{}{}started...",
            termion::cursor::Goto(1, 2),
            color::Fg(color::White)
        )
        .unwrap();

        stdout.flush().unwrap();
        start_timer();
    } else {
        write!(
            stdout,
            "{}{}{}Program aborted by user. ",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
            color::Fg(color::Red)
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

fn break_key_event(c: char) -> bool {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    for evt in stdin.keys() {
        let evt = evt.unwrap();

        if evt == Key::Char(c) {
            return true;
        } else if evt == Key::Ctrl('c') {
            return false;
        }

        stdout.flush().unwrap();
    }

    false
}

fn start_timer() {
    let stdout = stdout().into_raw_mode().unwrap();
    let mut stdout = stdout.lock();

    let start_time = Instant::now();
    let mut _timer_running = true;

    write!(
        stdout,
        "{}{}Press {}SPACE {}to {}stop {}the timer!",
        termion::cursor::Goto(1, 4),
        color::Fg(color::White),
        color::Fg(color::LightYellow),
        color::Fg(color::White),
        color::Fg(color::Red),
        color::Fg(color::White),
    )
    .unwrap();
    stdout.flush().unwrap();

    while _timer_running {
        if break_key_event(' ') {
            _timer_running = false;
            break;
        }
    }

    let elapsed_time = Instant::now() - start_time;
    write!(
        stdout,
        "{}{}Timer stopped...",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();

    write!(
        stdout,
        "{}Elapsed time: {}{:?}{} ",
        termion::cursor::Goto(1, 2),
        color::Fg(color::LightMagenta),
        elapsed_time,
        color::Fg(color::White)
    )
    .unwrap();

    stdout.flush().unwrap();
}
