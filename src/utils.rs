use dialoguer::console::Term;
use indicatif::{ProgressBar, ProgressStyle};

pub fn arrow_progress(steps: u64) -> ProgressBar {
    let pb = ProgressBar::new(steps);
    pb.set_style(ProgressStyle::with_template(
            // note that bar size is fixed unlike cargo which is dynamic
            // and also the truncation in cargo uses trailers (`...`)
            if Term::stdout().size().1 > 20 {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len} {wide_msg}"
            } else {
                "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
            },
        )
        .unwrap()
        .progress_chars("=> "));

    pb
}