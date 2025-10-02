mod utils;
mod core;
use clap::Parser;

use utils::logger::*;
use utils::args::Cli;
use utils::config::AppConfig;

fn main() {
    let cli = Cli::parse();
    let mut config = AppConfig::parse(&cli.config);
    set_global_level(config.system.log_level);
    cli.update_config_headers(&mut config);
    info_fmt!("{:?}", config);
}
