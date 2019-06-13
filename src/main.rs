extern crate rppal;

#[macro_use]
extern crate structopt;

use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

use structopt::StructOpt;

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
// const GPIO_LED: u8 = 18;
const STEPS_360: u32 = 4076;
const LIGHT_PIN: u8 = 18;
const FILTERS_PINS: [u8; 2] = [23,24];
const FEEDER_PINS : [u8; 4] = [4,17,27,22];

fn all_pins() -> Vec<u8> {
    let mut pins = Vec::new();
    pins.push(LIGHT_PIN);
    pins.extend_from_slice(&FILTERS_PINS);
    pins.extend_from_slice(&FEEDER_PINS);
    pins
}

fn seq() ->  Vec<[u8;4]> {
    vec![
       [1,0,0,1],
       [1,0,0,0],
       [1,1,0,0],
       [0,1,0,0],
       [0,1,1,0],
       [0,0,1,0],
       [0,0,1,1],
       [0,0,0,1]]
}

fn seq_rev() -> Vec<[u8;4]> {
    let mut seq = seq();
    seq.iter_mut().for_each(|e| e.reverse());
    seq
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Copy, Clone)]
struct Rotation {
    direction : Direction,
    angle : u32,
}

impl Rotation {
    fn from_angle(angle : i32) -> Self {
        Rotation {
            direction : if angle > 0 { Direction::Clockwise } else { Direction::CounterClockwise },
            angle : angle.abs() as u32, 
        }
    }
}

fn steps(rotation : Rotation) -> u32 {
    let of_360 = rotation.angle as f64 / 360.0;
    let steps = ((STEPS_360 as f64) * of_360).round();

    steps as u32
}

fn rotate(gpio : &mut Gpio, rotation : Rotation) {
    let seq = if rotation.direction == Direction::Clockwise { seq() } else { seq_rev() };
    for _ in 0..(steps(rotation) / seq.len() as u32 ) {
        for state in seq.iter() {
            for (index, value) in state.iter().enumerate() {
                let state = if *value == 0 { Level::Low } else { Level::High };
                gpio.write(FEEDER_PINS[index], state);
            }

            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn level_from_bool(b : bool) -> Level {
    if b { Level::High } else { Level::Low }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "lights")]
    toggle_lights : Option<bool>,

    #[structopt(long = "feed")]
    feed : Option<u32>,

    #[structopt(long = "rotate")]
    rotate : Option<i32>,

    #[structopt(long = "filters")]
    force_filters : Option<bool>,
}

fn toggle_filters(gpio : &mut Gpio, state : bool) {
    for p in FILTERS_PINS.iter() {
        gpio.write(*p, level_from_bool(state));
    }
}

fn main() {
    let opt = Opt::from_args();
    println!("opt: {:?}", opt);

    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = Gpio::new().unwrap();
    gpio.set_clear_on_drop(false);
    for p in all_pins().iter() {
        gpio.set_mode(*p, Mode::Output);
    }

    thread::sleep(Duration::from_millis(100));

    if let Some(toggle_lights) = opt.toggle_lights {
        gpio.write(LIGHT_PIN, level_from_bool(toggle_lights));
    }

    if let Some(filters) = opt.force_filters {
        toggle_filters(&mut gpio, filters);
    }

    if let Some(rotation) = opt.rotate {
        rotate(&mut gpio, Rotation::from_angle(rotation));
    }

    if let Some(feeder_rotation) = opt.feed {
        thread::sleep(Duration::from_millis(100));

        toggle_filters(&mut gpio, false);
        thread::sleep(Duration::from_secs(90));

        let rotation = feeder_rotation as i32;
        rotate(&mut gpio, Rotation::from_angle(rotation));
        rotate(&mut gpio, Rotation::from_angle(-rotation));

        for p in FEEDER_PINS.iter() {
            gpio.write(*p, Level::Low);
        }

        thread::sleep(Duration::from_secs(90));
        toggle_filters(&mut gpio, true);
    }

    for p in FEEDER_PINS.iter() {
        gpio.write(*p, Level::Low);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foo() {
        assert_eq!(STEPS_360, steps(Rotation::from_angle(360)));
        assert_eq!(STEPS_360 / 2, steps(Rotation::from_angle(180)));
        assert_eq!(STEPS_360 / 2, steps(Rotation::from_angle(-180)));
        assert_eq!(STEPS_360 / 360, steps(Rotation::from_angle(-1)));
    }
}
