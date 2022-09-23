use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{
    Event,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
    Config, EventKind,
    event::{ModifyKind, DataChange},
};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;

use moynihan::parser::{self, NginxErrLog};
use moynihan::mastodon::{self, toot};

static LOG_PATH: &str = "/var/log/nginx/error.log";

fn main() {
    let log_path = Path::new(LOG_PATH);
    futures::executor::block_on(async{
        if let Err(e) = async_watch(log_path).await {
            println!("error: {:?}", e);
        }
    });
}

fn get_latest_log(log_path: &Path) -> String {
    let log_file= BufReader::new(File::open(log_path).unwrap());
    let log_iter = log_file.lines().map(|l| l.unwrap());
    let last_log = match log_iter.last() {
        Some(s) => s,
        None => panic!("Invalid log file"),
    };

    last_log
}

fn event_handler(event: Event) {
    // Event { kind: Modify(Data(Any))
    if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
        let last_log = get_latest_log(Path::new(LOG_PATH));
        let parsed_log: NginxErrLog = parser::parser(last_log);
        let msg = String::from(format!(
            "Payload detected!\n{}\n{}\nfrom {}"
            , parsed_log.time
            , parsed_log.payload
            , parsed_log.client
        ));
        match toot(msg) {
            Ok(_) => println!("send!"),
            Err(_) => panic!("Cannot access mastodon instance"),
        };
    }
}

// https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs
fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    }, Config::default())?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => event_handler(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static LOG_PATH: &str = "./test/test.log";
    #[test]
    fn get_latest_log_test() {
        let last_log = get_latest_log(Path::new(LOG_PATH));
        assert_eq!(last_log, "2022/09/22 20:06:54 [error] 1036243#1036243: *3757626 no live upstreams while connecting to upstream, client: 192.168.11.4, server: , request: \"GET /piyo HTTP/1.1\", upstream: \"http://localhost/\", host: \"192.168.11.1\"")
    }
}
