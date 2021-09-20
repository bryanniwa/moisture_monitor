use std::path::PathBuf;

pub enum Led {
    Led0,
    Led1,
    Led2,
    Led3,
}

impl Led {
    fn get_path(&self) -> PathBuf {
        let base = String::from("/sys/class/leds/beaglebone:green:usr");
        let path = base + match self {
            Led::Led0 => "0",
            Led::Led1 => "1",
            Led::Led2 => "2",
            Led::Led3 => "3",
        };
        PathBuf::from(path)
    }
}

pub fn turn_on_led(led: &Led) {
    let full_path = led.get_path();
    let display = full_path.display();
    println!("{}", display)
}