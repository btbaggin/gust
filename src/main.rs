#![feature(maybe_uninit_array_assume_init)]

mod assets;
mod job_system;
mod pooled_cache;
mod logger;

fn main() {
    logger::init();

    assets::initialize_asset_cache();

    let (queue, notify) = job_system::start_job_system();

}
