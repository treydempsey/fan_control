# Raspberry Pi PWM Fan Control in Rust

This is a temperature speed control daemon for your 4 pin PWM fan on a Raspberry Pi.

## Motivation

I have a Raspberry Pi 4b I use with [Hyperion](https://hyperion-project.org/forum/).
At times it can get very hot, ~70C. I bought a case with integrated fans, but the small
fans eventually became noisy. I Switched to Noctua NF-A6x25 5V 60mm fan. At full speed
it is also quite noisy. Using Python code from [here](https://blog.driftking.tw/en/2019/11/Using-Raspberry-Pi-to-Control-a-PWM-Fan-and-Monitor-its-Speed/) as inspiration, I set out to build a program to control the fan.

## Building from Source

* Install [rustup](https://rustup.rs/)
* Clone this git repository
    * ```git clone https://github.com/treydempsey/fan_control```
* Build the code
    * ```cargo build --release``` 
* Install the binary
    * ```sudo cp target/release/fan_control /usr/local/bin```
* Install the systemd unit file
    *

        ```bash
        sudo cp fan_control.service /lib/systemd/system
        sudo systemctl enable fan_control
        sudo systemctl start fan_control
        ````


## Binaries

Prebuilt binaries are avalible in the GitHub [releases](https://github.com/treydempsey/fan_control/releases) for this project.
