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
use chess::{Board, ChessMove};
use std::str::{FromStr, SplitWhitespace};

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

    let mut board: Board = Board::default();

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
            },
            /*
             * position [fen <fenstring> | startpos ]  moves <move1> .... <movei>
             * A FEN string is a chess board notation. Information can be found here:
             * https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
             */
            "position" => {
                // Grab the next argument in the command. Looking for 'startpos' or 'fen'.
                let position_option = split.next().unwrap();

                match position_option {
                    "fen" => {
                        // Mutable string used to build out the FEN string from the command arguments.
                        let mut fen_string = "".to_owned();

                        // Loop until there are no more arguments or the 'moves' argument is found.
                        loop {
                            let fen_split = split.next();

                            match fen_split {
                                Some(x) => {
                                    if x == "moves" {
                                        board = Board::from_str(&fen_string).expect("Invalid fen string provided!");
                                        do_moves(&mut board, split);
                                        break;
                                    }

                                    if fen_string == "" {
                                        fen_string = x.to_string();
                                    } else {
                                        fen_string = [fen_string, x.to_string()].join(" ");
                                    }
                                },
                                None => board = Board::from_str(&fen_string).expect("Invalid fen string provided!"),
                            }
                        }
                    },
                    "startpos" => {
                        board = Board::default();
                        do_moves(&mut board, split);
                    }
                    _ => error!("Unrecognized position option {}", position_option),
                }
                println!("Current board position: {}", board);
            }
            "quit" => break,
            _ => info!("Received input: {}", trimmed_line)
        }
    }
}

fn do_moves(board: &mut Board, mut moves: SplitWhitespace) {
    let chess_move = moves.next();

    loop {
        match chess_move {
            Some(x) => {
                if x == "moves" {
                    continue;
                }

                let mv = ChessMove::from_san(board, x);
                if mv.is_ok() {
                    board.make_move(mv.unwrap(), board);
                } else {
                    error!("There was an error while attempting to make the chess move: {}", x);
                }
            },
            None => break,
        }
    }

    println!("{}", board);
}
