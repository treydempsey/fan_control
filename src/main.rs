mod config;
mod fan_control;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::Clap;
use config::Config;
use env_logger;
use fan_control::FanControl;


fn main() -> Result<()> {
    env_logger::init();

    let config = Config::parse();

    let mut fan_control = FanControl::new(&config)?;
    let wait_time = Duration::from_millis(config.wait_time);
    loop {
        fan_control.run()?;
        thread::sleep(wait_time);
    }
     
    #[allow(unreachable_code)]
    Ok(())
}
