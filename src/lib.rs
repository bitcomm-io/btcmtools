pub mod command;


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







pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        command::main();

    }
}
