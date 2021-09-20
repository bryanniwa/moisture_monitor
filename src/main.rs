mod led_controller;

fn main() {
    let led = led_controller::Led::Led0;
    let led1 = led_controller::Led::Led1;
    let led2 = led_controller::Led::Led2;
    let led3 = led_controller::Led::Led3;
    led_controller::turn_on_led(&led);
    led_controller::turn_on_led(&led1);
    led_controller::turn_on_led(&led2);
    led_controller::turn_on_led(&led3)
}
