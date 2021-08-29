use clap::Clap;


/// Temperature speed control of a 4 pin PWM fan
#[derive(Clap)]
#[clap(version = "v0.2.0", author = "Trey Dempsey <trey.dempsey@gmail.com>")]
pub struct Config {
    /// GPIO/BCM pin number. Defaults to the hardware PWM0 pin. https://pinout.xyz/pinout/pin12_gpio18
    #[clap(short, long, validator = validate_pwm_pin, default_value = "18")]
    pub fan_pin: u8,

    /// Delay in ms between each temperature check and fan speed update.
    #[clap(short, long, validator = validate_integer, default_value = "1000")]
    pub wait_time: u64,

    /// PWM frequency. The Intel fan standard defines this as between 21kHz and 28kHz. Typically
    /// this value is 25kHz.
    #[clap(short, long, validator = validate_between_21000f_28000f, default_value = "25000.0")]
    pub pwm_freq: f64,

    /// Minimum temperature. Values below this turn off the fan.
    #[clap(long, validator = validate_float, default_value = "40.0")]
    pub min_temp: f64,

    /// Minimum dead band
    #[clap(long, validator = validate_float, default_value = "5.0")]
    pub min_temp_dead_band: f64,

    /// Maximum temperature. Values above this turn on the fan to maximum.
    #[clap(long, validator = validate_float, default_value = "60.0")]
    pub max_temp: f64,

    /// PWM duty cycle percentage for the low speed fan.
    #[clap(long, validator = validate_between_0f_100f, default_value = "1.0")]
    pub fan_low: f64,

    /// PWM duty cycle percentage for the high speed fan.
    #[clap(long, validator = validate_between_0f_100f, default_value = "100.0")]
    pub fan_high: f64,

    /// PWM duty cycle percentage for the off fan.
    #[clap(long, validator = validate_between_0f_100f, default_value = "0.0")]
    pub fan_off: f64,

    /// PWM duty cycle percentage for the maximum speed fan.
    #[clap(long, validator = validate_between_0f_100f, default_value = "100.0")]
    pub fan_max: f64,
}

fn validate_integer(v: &str) -> Result<(), String> {
    match v.parse::<u64>() {
        Ok(_) => Ok(()),
        _ => Err(String::from("The value must be an integer")),
    }
}

fn validate_pwm_pin(v: &str) -> Result<(), String> {
    match v.parse::<u8>() {
        Ok(v) if v == 18 || v == 19 => Ok(()),
        _ => Err(String::from("The hardware PWM pin must be one of BCM GPIO 18 or 19")),
    }
}

fn validate_float(v: &str) -> Result<(), String> {
    match v.parse::<f64>() {
        Ok(_) => Ok(()),
        _ => Err(String::from("The value must be a floating point number with the decimal point")),
    }
}

fn validate_between_0f_100f(v: &str) -> Result<(), String> {
    match v.parse::<f32>() {
        Ok(v) if v >= 0.0 || v <= 100.0 => Ok(()),
        _ => Err(String::from("The value must be between 0.0 and 100.0")),
    }
}

fn validate_between_21000f_28000f(v: &str) -> Result<(), String> {
    match v.parse::<f32>() {
        Ok(v) if v >= 21000.0 || v <= 28000.0 => Ok(()),
        _ => Err(String::from("The value must be between 21000.0 and 28000.0")),
    }
}
