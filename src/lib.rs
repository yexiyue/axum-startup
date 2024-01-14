use clap::{Parser, Subcommand};
mod new;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Startup {
    /// Debug mode
    #[clap(short,long,action=clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Command,
}

// 定义子命令
#[derive(Debug, Subcommand)]
enum Command {
    /// Start a new project
    New,
}

// 为Startup添加run方法，在main函数中调用
impl Startup {
    pub fn start() -> Self {
        Startup::parse()
    }

    pub fn run(&self) {
        // 初始化tracing日志
        let level = if self.debug > 0 {
            tracing::Level::TRACE
        } else {
            tracing::Level::INFO
        };
        tracing_subscriber::fmt().with_max_level(level).init();
        // 运行命令
        self.command.run();
    }
}

// 为command添加run方法，不同的命令调不同的方法
impl Command {
    fn run(&self) {
        match self {
            Command::New => new::create_new_project(),
        }
    }
}
