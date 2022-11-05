pub mod cron;
mod git;
pub mod gql_client;
pub mod init;
use init::init;
extern crate log;

use crate::cron::run_cron;
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() {
    init();
    let temp1 = &dotenv::var("GITHUB_PERSONAL_ACCESS_TOKEN")
        .expect("GITHUB_PERSONAL_ACCESS_TOKEN not found");
    let temp2 = &dotenv::var("GRAPHQL_ENDPOINT")
        .expect("GRAPHQL_ENDPOINT not found");
    let temp3 = &dotenv::var("GITHUB_USERNAME")
        .expect("GITHUB_USERNAME not found");
    let temp4 =
        &dotenv::var("USER_AGENT").expect("USER_AGENT not found");
    println!(
        "Temps: {:?} - {:?} - {:?} - {:?}",
        temp1, temp2, temp3, temp4
    );
    let sched = JobScheduler::new().await;
    let sched = sched.unwrap();
    run_cron(sched).await.unwrap();
}
