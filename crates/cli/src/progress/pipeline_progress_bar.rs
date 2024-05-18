use indicatif::{ProgressBar, ProgressStyle};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use std::sync::{Arc, RwLock};
use std::thread::sleep;

pub fn run_progress_bar_for_pipeline(pipeline_progress: Arc<RwLock<PipelineProgress>>) {
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::with_template("{msg} [{bar:40.green/cyan}] {percent}%").unwrap());

    while !pipeline_progress.read().unwrap().is_ready() {
        sleep(std::time::Duration::from_millis(5));
    }

    let steps = pipeline_progress.read().unwrap().get_combined_max();

    while !pipeline_progress.read().unwrap().is_finished() {
        let current_step = pipeline_progress.read().unwrap().get_combined_value();
        let current_name = pipeline_progress.read().unwrap().get_current_individual_name();
        let current_percentage = pipeline_progress.read().unwrap().get_current_individual_percentage(current_name.clone());
        bar.set_message(format!("[step: {}/{}] {}", current_step + 1, steps, current_name));
        bar.set_position(current_percentage as u64);
        sleep(std::time::Duration::from_millis(10));
    }

    bar.finish();

    println!();
}
