extern crate rppal;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

//# GPIO17,GPIO22,GPIO23,GPIO24
//StepPins = [4,17,27,22]
//
//# Set all pins as output
//for pin in StepPins:
//  print "Setup pins"
//  GPIO.setup(pin,GPIO.OUT)
//  GPIO.output(pin, False)
//
//# Define advanced sequence
//# as shown in manufacturers datasheet
//Seq = [[1,0,0,1],
//       [1,0,0,0],
//       [1,1,0,0],
//       [0,1,0,0],
//       [0,1,1,0],
//       [0,0,1,0],
//       [0,0,1,1],
//       [0,0,0,1]]

//for x in range(0, 4*4*128):
//
//  print StepCounter,
//  print Seq[StepCounter]
//
//  for pin in range(0, 4):
//    xpin = StepPins[pin]
//    if Seq[StepCounter][pin]!=0:
//      print " Enable GPIO %i" %(xpin)
//      GPIO.output(xpin, True)
//    else:
//      GPIO.output(xpin, False)
//
//  StepCounter += StepDir

// The GPIO module uses BCM pin numbering. BCM GPIO 18 is tied to physical pin 12.
const GPIO_LED: u8 = 18;

fn main() {
    let pins : [u8; 4] = [4,17,27,22];

    let seq =
        [[1,0,0,1],
       [1,0,0,0],
       [1,1,0,0],
       [0,1,0,0],
       [0,1,1,0],
       [0,0,1,0],
       [0,0,1,1],
       [0,0,0,1]];

    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();
    for p in pins.iter() {
        gpio.set_mode(*p, Mode::Output);
    }

    for p in pins.iter() {
        println!("state is: {:?}", gpio.read(*p));
    }

    thread::sleep(Duration::from_millis(1000));

    println!("clear");
    for p in pins.iter() {
        gpio.write(*p, Level::Low);
    }

    thread::sleep(Duration::from_millis(1000));

    loop {
        for state in seq.iter() {
            println!("now setting: {:?}", state);
            for (index, value) in state.iter().rev().enumerate() {
                let state = if *value == 0 { Level::Low } else { Level::High };
                gpio.write(pins[index], state);
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}