use indicatif::{ProgressBar, ProgressStyle};
use picturify_pipeline::common::pipeline_progress::PipelineProgress;
use std::sync::{Arc, RwLock};
use std::thread::sleep;

pub fn run_progress_bar_for_pipeline(pipeline_progress: Arc<RwLock<PipelineProgress>>) {
    let bar = ProgressBar::new(100);
    bar.set_style(ProgressStyle::with_template("{msg} [{bar:40.green/cyan}] {percent}%").unwrap());

    let mut processor_names = pipeline_progress.read().unwrap().get_individual_names();

    while processor_names.is_empty() {
        processor_names = pipeline_progress.read().unwrap().get_individual_names();

        sleep(std::time::Duration::from_millis(5));
    }

    processor_names.iter().for_each(|name| loop {
        let percentage = pipeline_progress
            .read()
            .unwrap()
            .get_individual_percentage(name.clone());
        bar.set_message(format!("Running processor: {}", name));
        bar.set_position(percentage as u64);
        if percentage == 100.0 {
            break;
        }
    });

    bar.finish();
}
