mod adc;

use std::{thread, time};

const POTENTIOMETER: &str = "/sys/bus/iio/devices/iio:device0/in_voltage0_raw";

pub fn run() {
    check_potentiometer()
}

fn check_potentiometer() {
    loop {
        let adc_val = adc::read_adc(POTENTIOMETER);
        match adc_val {
            Ok(v) => println!("ADC: {}", v),
            Err(_) => println!("Problem reading adc file")
        }
        thread::sleep(time::Duration::from_secs(1))
    }
}