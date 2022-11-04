#![allow(unused)]
use crate::gql_client::CustomizedGqlClient;
use anyhow::Result;
use log::{error, info, warn};
use std::time::Duration;
use tokio::time;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::git::Git;

pub(crate) async fn run_cron(mut sched: JobScheduler) -> Result<()> {
    #[cfg(feature = "signal")]
    sched.shutdown_on_ctrl_c();

    sched.set_shutdown_handler(Box::new(|| {
        Box::pin(async move {
            info!("Shut down done");
        })
    }));

    // let mut job = Job::new_async("0 0 14 ? * * *", |uuid, mut l| {
    let mut job =
        Job::new_async("3 5,9,11,13,15 * ? * * *", |uuid, mut l| {
            Box::pin(async move {
                info!("I run async, id {:?}", uuid);

                // Initialize GQL Client
                let client = CustomizedGqlClient::new_client();
                // Trigger action to get latest commits of repos
                let result = Git::get_latest_commits(
                    &client,
                    &dotenv::var("GITHUB_USERNAME")
                        .expect("Username not found"),
                    Some(2),
                    None,
                )
                .await
                .unwrap();
                info!("Response: {:?}", "OK");

                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => {
                        info!("Next time is {:?}", ts)
                    }
                    _ => warn!("Could not get next tick for 59s job"),
                }
            })
        })
        .unwrap();

    let job_clone = job.clone();
    let js = sched.clone();
    info!("Job id {:?}", job.guid());
    job.on_start_notification_add(&sched, Box::new(move |job_id, notification_id, type_of_notification| {
        let job_clone = job_clone.clone();
        let js = js.clone();
        Box::pin(async move {
            info!("Job {:?} ran on start notification {:?} ({:?})", job_id, notification_id, type_of_notification);
        })
    })).await?;

    job
        .on_done_notification_add(
            &sched,
            Box::new(|job_id, notification_id, type_of_notification| {
                Box::pin(async move {
                    info!(
                        "Job {:?} completed and ran notification {:?} ({:?})",
                        job_id, notification_id, type_of_notification
                    );
                })
            }),
        )
        .await?;

    let four_s_job_guid = job.guid();
    sched.add(job).await?;

    let start = sched.start().await;
    if start.is_err() {
        error!("Error starting scheduler");
        return Ok(());
    }

    loop {
        sched.tick();
        time::sleep(Duration::from_millis(1000));
    }

    Ok(())
}
