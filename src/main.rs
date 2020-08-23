use log::{error, info, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    Handle,
};
use std::io::{self, BufRead};

fn get_logger_config(filter: LevelFilter) -> Config {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "info string {d} {l} - {m}{n}",
        )))
        .target(Target::Stdout)
        .build();

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build("log/run.log")
        .unwrap();

    return Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                .build(filter),
        )
        .unwrap();
}

fn init_logger() -> Handle {
    return log4rs::init_config(get_logger_config(LevelFilter::Info)).unwrap();
}

fn main() {
    let log_handle = init_logger();

    info!("Engine started");

    loop {
        let mut line = String::new();

        let stdin = io::stdin();
        stdin.lock().read_line(&mut line).unwrap();

        let trimmed_line = line.as_str().trim();
        let mut split = trimmed_line.split_whitespace();

        match split.next().unwrap() {
            "isready" => println!("readyok"),
            "debug" => {
                let debug_option = split.next().unwrap();
                match debug_option {
                    "on" => {
                        log_handle.set_config(get_logger_config(LevelFilter::Debug));
                        info!("Enabled debugging")
                    },
                    "off" => {
                        log_handle.set_config(get_logger_config(LevelFilter::Info));
                        info!("Disabled debugging")
                    },
                    _ => error!("Unrecognized debug option {}", debug_option),
                }
            }
            "quit" => break,
            _ => info!("Received input: {}", trimmed_line)
        }
    }
}
