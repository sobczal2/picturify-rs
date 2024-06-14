use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use picturify_movie::progress::{MovieProgress, ProgressStage};

pub fn run_progress_bar_for_movie(movie_progress: Arc<RwLock<MovieProgress>>) {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner:.green}")
            .unwrap(),
    );
    bar.set_message("Probing video");
    while movie_progress.read().unwrap().get_stage() == ProgressStage::Probe {
        sleep(Duration::from_millis(10));
    }
    bar.finish();

    let bar = ProgressBar::new(movie_progress.read().unwrap().get_max() as u64);
    bar.set_style(
        ProgressStyle::with_template("{msg} [{bar:40.green/cyan}] frame {pos}/{len} ({percent}%)")
            .unwrap(),
    );
    bar.set_message("Processing video");
    while movie_progress.read().unwrap().get_stage() == ProgressStage::Process {
        bar.set_position(movie_progress.read().unwrap().get() as u64);
        sleep(Duration::from_millis(10));
    }
    bar.finish();

    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{msg} {spinner:.green}")
            .unwrap(),
    );
    bar.set_message("Merging video");
    while movie_progress.read().unwrap().get_stage() == ProgressStage::Merge {
        sleep(Duration::from_millis(10));
    }
    bar.finish();

    println!();
}
