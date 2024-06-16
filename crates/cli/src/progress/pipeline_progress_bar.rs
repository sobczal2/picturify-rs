use std::thread::sleep;
use colored::Colorize;

use indicatif::{ProgressBar, ProgressStyle};

use picturify_pipeline::common::pipeline_progress::PipelineProgress;

pub fn run_progress_bar_for_pipeline(pipeline_progress: PipelineProgress) {
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::with_template("{msg} [{bar:40.green/cyan}] {percent}%").unwrap());

    while !pipeline_progress.is_ready() {
        sleep(std::time::Duration::from_millis(5));
    }

    let steps = pipeline_progress.get_combined_max();

    while !pipeline_progress.is_finished() {
        let current_step = pipeline_progress.get_combined_value();
        let current_name = pipeline_progress.get_current_individual_name();
        let current_percentage = pipeline_progress.get_current_individual_percentage();
        bar.set_message(format!(
            "{} {}",
            format!(
                "[STEP: {}/{}]",
                current_step + 1,
                steps,
            ).bold(),
            current_name,
        ));
        bar.set_position(current_percentage as u64);
        sleep(std::time::Duration::from_millis(10));
    }

    let last_name = pipeline_progress.get_last_individual_name();
    bar.set_message(format!(
        "{} {}",
        format!(
            "[STEP: {}/{}]",
            steps,
            steps,
        ).bold(),
        last_name,
    ));
    bar.set_position(100);

    bar.finish();

    println!();
}
