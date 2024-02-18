pub mod imcmd;
pub mod pid;

use once_cell::sync::Lazy;
use slog::{o, Logger,Drain};
// use sloggers::{Build, terminal::TerminalLoggerBuilder};

// 定义全局 Logger 变量
pub static LOGGER: Lazy<Logger> = Lazy::new(build_logger);

// 
fn build_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());
    logger
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bitcomm")]
/// bitcomm instant message server
pub enum BitcommOpt {
    /// Start instant message server and web admin server
    #[structopt(name = "start", help = "Start instant message server and web admin server")]
    StartServer,
    /// Stop  instant message server and web admin server
    #[structopt(name = "stop", help = "Stop  instant message server and web admin server")]
    StopServer,
}




