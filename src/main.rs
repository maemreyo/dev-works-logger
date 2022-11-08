#![allow(unused)]
mod modules;
mod utils;

use modules::{
    cron::run_cron,
    twitter::{common::content_generator, Twitter},
};
use utils::init::init;
extern crate log;

// use crate::cron::run_cron;
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() {
    init();
    let sched = JobScheduler::new().await;
    let sched = sched.unwrap();
    run_cron(sched).await.unwrap();
}
