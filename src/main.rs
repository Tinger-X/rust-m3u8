mod utils;
mod core;
use clap::Parser;

use utils::args::Cli;
use utils::config::AppConfig;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
    let mut app_config = AppConfig::parse(&cli.config).unwrap();
    cli.update_config_headers(&mut app_config);
    println!("{:?}", app_config);
}
