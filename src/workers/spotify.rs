use apalis::{layers::retry::RetryPolicy, prelude::*};
use apalis_cron::CronContext;
use apalis_cron::CronStream;
use apalis_cron::Schedule;
use chrono::Utc;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
struct Spotify;

pub async fn register(ctrl_c: async_channel::Receiver<()>) -> Result<(), anyhow::Error> {
    let schedule = Schedule::from_str("0 * * * * *").unwrap();
    let worker = WorkerBuilder::new("spotify-min-update-worker")
        .retry(RetryPolicy::retries(1))
        .backend(CronStream::new(schedule))
        .build_fn(update_spotify_job_handler);

    Monitor::new()
        .register(worker)
        .on_event(|e| log::debug!("Spotify Worker event: {e:?}"))
        .run_with_signal(async {
            ctrl_c.recv().await.ok();
            log::info!("Spotify Worker Shutting down");
            Ok(())
        })
        .await?;

    Ok(())
}

async fn update_spotify_job_handler(_job: Spotify, _ctx: CronContext<Utc>) {
    log::info!("Running Spotify Update Job");
    match crate::utils::fetch_spotify_endpoint().await {
        Ok(_) => log::debug!("Successfully Polled Spotify Update API"),
        Err(err) => log::error!("Failed to poll Spotify Update API: {err}"),
    };
}
