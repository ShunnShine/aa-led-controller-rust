mod led_controller;
mod plan;
use std::{sync::mpsc::Receiver, thread};
use plan::Plan;

pub enum Message {
    LedsOn(Vec<u8>),
    LedsOff(Vec<u8>),
    AllLedsOn,
    AllLedsOff,
    LedsBlink(Vec<u8>),
}

const NUMBER_OF_LEDS: u8 = 4 * 32;

pub fn start(receive_channel: Receiver<Message>) -> () {
    let led_controller_sender = set_up_led_controller();

    let mut new_plan;
    for message in receive_channel {
        match message {
            Message::LedsOn(leds) => {
                for led in leds {
                    if led < NUMBER_OF_LEDS {
                        new_plan.add(led);
                    }
                }
            }
            Message::LedsOff(leds) => {
                for led in leds {
                    if led < NUMBER_OF_LEDS {
                        new_plan.remove(led);
                    }
                }
            }
            Message::AllLedsOn => {
                new_plan.allOn();
            }
            Message::AllLedsOff => {
                new_plan.allOff();
            }
            Message::LedsBlink(leds) => {
                new_plan.allOff();
                for led in leds {
                    if led < NUMBER_OF_LEDS {
                        new_plan.add(led);
                    }
                }
                todo!();
            }
        };
        match led_controller_sender.send(new_plan) {
            Err(e) => { panic!("[Error] Unable to send new plan to LED Controller.") }
            _ => {}
        };
    }
    panic!("[Error] Lost all sending channels in Plan Generator.")
}

fn set_up_led_controller() -> mpsc::Sender<> {
    let (send_end, receive_end) = mpsc::channel();
    thread::spawn(move || {
        led_controller::start(receive_end, );
    });
    send_end
}