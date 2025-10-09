use apalis::{layers::retry::RetryPolicy, prelude::*};
use apalis_cron::CronContext;
use apalis_cron::CronStream;
use apalis_cron::Schedule;
use chrono::Utc;
use std::str::FromStr;

use crate::utils::run_station_parser;

#[derive(Debug, Default, Clone)]
struct NrStationParser;

pub async fn register(ctrl_c: async_channel::Receiver<()>) -> Result<(), anyhow::Error> {
    let schedule = Schedule::from_str("0 0 0 * * *").unwrap();
    let worker = WorkerBuilder::new("nr-station-parser-daily-worker")
        .retry(RetryPolicy::retries(1))
        .backend(CronStream::new(schedule))
        .build_fn(parser_job_handler);

    Monitor::new()
        .register(worker)
        .on_event(|e| log::debug!("NR Station Parser Worker event: {e:?}"))
        .run_with_signal(async {
            ctrl_c.recv().await.ok();
            log::info!("NR Station Parser Worker Shutting down");
            Ok(())
        })
        .await?;

    Ok(())
}

async fn parser_job_handler(_job: NrStationParser, _ctx: CronContext<Utc>) {
    log::info!("NR Station Parser Job");
    run_station_parser();
}
