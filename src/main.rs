use std::{thread::sleep, time::Duration};

use log::info;
use time::{macros::format_description, UtcOffset};
use tracing::{event, instrument, span, Level};
use tracing_subscriber::fmt::time::OffsetTime;

#[instrument]
fn expensive_work(secs: u64) {
    info!("doing expensive work");
    sleep(Duration::from_secs(secs));
    info!("done with expensive work");
}

fn main() {
    init_log();
    event!(Level::INFO, "event"); // 在 span 的上下文之外记录一个 Leval 为 INFO 的 event

    let span = span!(Level::INFO, "span");
    let _enter = span.enter();

    event!(Level::INFO, "event"); // 在 span 的上下文内记录 event

    info!("something with info level"); // 也能够应用和 log 雷同的模式记录 event
}
fn init_log() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt().with_timer(local_time).init();
}
