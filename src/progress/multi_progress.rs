use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub fn setup_progress() -> MultiProgress {
    MultiProgress::new()
}

pub fn add_progress(mp: &MultiProgress, total_steps: u64, text: &str) -> ProgressBar {
    let pb = mp.add(ProgressBar::new(total_steps));

    let style =
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ");

    pb.set_style(style);
    pb.set_message(format!("{}", text));

    pb
}
