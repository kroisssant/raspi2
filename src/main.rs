use std::{thread::sleep, time};

use rppal::gpio::{Error, Gpio, OutputPin};


static GPIOPINS: [u8; 27] = [2, 3, 4, 17, 27, 22, 10, 9, 11, 0, 5, 6, 13, 19, 26, 14, 15, 18, 23, 24, 25, 8, 7, 1, 12, 16, 21];

fn main() {
    let gpio = Gpio::new().unwrap();
    let mut pin1 = gpio.get(2).unwrap().into_output();
    let mut pin2 = gpio.get(3).unwrap().into_output();
    let mut pin3 = gpio.get(4).unwrap().into_output();
    let mut gnd1 = gpio.get(17).unwrap().into_output();
    let mut gnd2 = gpio.get(27).unwrap().into_output();

    let ten_millis = time::Duration::from_millis(10);
    
    while true {
        // p1
        gnd1.set_low();
        gnd2.set_high();
        pin1.set_high();
        pin2.set_low();
        pin3.set_low();
        sleep(ten_millis);
        gnd1.set_low();
        gnd2.set_high();
        pin2.set_high();
        pin1.set_low();
        pin3.set_low();
        sleep(ten_millis);
        gnd1.set_low();
        gnd2.set_high();
        pin3.set_high();
        pin1.set_low();
        pin2.set_low();
        sleep(ten_millis);

        gnd2.set_low();
        gnd1.set_high();
        pin1.set_high();
        pin2.set_low();
        pin3.set_low();
        sleep(ten_millis);
        gnd2.set_low();
        gnd1.set_high();
        pin2.set_high();
        pin1.set_low();
        pin3.set_low();
        sleep(ten_millis);
        gnd2.set_low();
        gnd1.set_high();
        pin3.set_high();
        pin1.set_low();
        pin2.set_low();
        sleep(ten_millis);
    }

}
