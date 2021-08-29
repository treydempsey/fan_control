use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

use anyhow::{Context, Result};
use crate::config::Config;
use log::debug;
use rppal::pwm::{Channel, Polarity, Pwm};

pub struct FanControl<'c> {
    pub config: &'c Config,
    thermal_reader: BufReader<File>,
    temperature: f64,
    outside_dead_band_higher: bool,
    fan_pwm: Pwm,
}

impl<'c> FanControl<'c> {
    pub fn new(config: &'c Config) -> Result<Self> {
        let thermal_file = File::open("/sys/class/thermal/thermal_zone0/temp").context("unable to open /sys/class/thermal/thermal_zone0/temp")?;
        let thermal_reader = BufReader::new(thermal_file);
        let mut fan_pwm = Pwm::with_frequency(Channel::Pwm0, config.pwm_freq, 0.0, Polarity::Normal, true).context(format!("unable to create PWM channel for pin {}", config.fan_pin))?;
        fan_pwm.set_reset_on_drop(true);

        Ok(FanControl { config, thermal_reader, temperature: 0.0, outside_dead_band_higher: true, fan_pwm })
    }

    pub fn run(&mut self) -> Result<()> {
        self.get_cpu_temperature()?;
        self.handle_dead_zone();
        self.handle_fan_speed()?;

        Ok(())
    }

    fn get_cpu_temperature(&mut self) -> Result<()> {
        let mut line = String::new();
        // seek(0) to get a new reading
        self.thermal_reader.seek(SeekFrom::Start(0))?;
        self.thermal_reader.read_line(&mut line)?;
        
        // Trim off trailing new line characters
        let len = line.trim_end_matches(&['\r', '\n'][..]).len();
        line.truncate(len);

        self.temperature = line.parse::<f64>().context(format!("couldn't parse string into number {}", line))? / 1000.0;
        debug!("Temperature is {:.2}°C", self.temperature);

        Ok(())
    }

    fn handle_dead_zone(&mut self) {
        if self.temperature > (self.config.min_temp + self.config.min_temp_dead_band / 2.0) {
            self.outside_dead_band_higher = true;
        }
        else if self.temperature < (self.config.min_temp - self.config.min_temp_dead_band / 2.0) {
            self.outside_dead_band_higher = false;
        }
    }

    fn handle_fan_speed(&mut self) -> Result<()> {
        // Turn off the fan if lower than lower dead band 
        if self.outside_dead_band_higher == false {
            debug!("Fan off");
            self.set_fan_duty(self.config.fan_off)?;
        }
        // Run fan at calculated speed if being in or above dead zone not having passed lower dead band    
        else if self.outside_dead_band_higher == true && self.temperature < self.config.max_temp {
            let step = (self.config.fan_high - self.config.fan_low) / (self.config.max_temp - self.config.min_temp);
            self.temperature = self.temperature - self.config.min_temp;
            self.set_fan_duty(self.config.fan_low + self.temperature * step)?;
        }
        // Set fan speed to MAXIMUM if the temperature is above MAX_TEMP
        else if self.temperature > self.config.max_temp {
            debug!("Maximum fan");
            self.set_fan_duty(self.config.fan_max)?;
        }

        Ok(())
    }

    fn set_fan_duty(&mut self, duty: f64) -> Result<()> {
        // Restrict and normalize to 0.0 - 1.0
        let duty = if duty > 100.0 {
            1.0
        }
        else if duty < 0.0 {
            0.0
        }
        else {
            duty / 100.0
        };

        debug!("Setting fan duty to {:.3}", duty);
        self.fan_pwm.set_duty_cycle(duty).context("could not set the PWM duty cycle")?;

        Ok(())
    }
}
