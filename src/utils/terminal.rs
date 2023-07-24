use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};

pub fn arrow_progress(steps: u64) -> ProgressBar {
    let pb = ProgressBar::new(steps);
    pb.set_style(
        ProgressStyle::with_template(
            // note that bar size is fixed unlike cargo which is dynamic
            // and also the truncation in cargo uses trailers (`...`)
            if Term::stdout().size().1 > 20 {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
            } else {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
            },
        )
        .unwrap()
        .progress_chars("=> "),
    );

    pb
}

#[macro_export]
macro_rules! print_err {
    ($fmt:literal) => (println!("\x1B[1;31merror\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[1;31merror\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
#[macro_export]
macro_rules! print_solution {
    ($fmt:literal) => (println!("\x1B[38;5;227mhint\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;227mhint\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
#[macro_export]
macro_rules! print_success {
    ($fmt:literal) => (println!("\x1B[38;5;46minfo\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;46minfo\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
#[macro_export]
macro_rules! print_info {
    ($fmt:literal) => (println!("\x1B[38;5;57minfo\x1B[0m: {}", $fmt));
    ($fmt:literal, $($arg:expr),*) => (println!("\x1B[38;5;57minfo\x1B[0m: {}", format_args!($fmt, $($arg),*)));
}
