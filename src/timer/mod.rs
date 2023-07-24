mod annex;

use std::io::{stdout, Write};
use termion::color;
use termion::input::{MouseTerminal, };
use termion::raw::IntoRawMode;

use self::annex::{break_key_event, start_timer};

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
