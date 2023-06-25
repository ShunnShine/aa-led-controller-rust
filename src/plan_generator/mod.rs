mod led_controller;
mod plan;
use std::sync::mpsc;
use std::{sync::mpsc::Receiver, thread};
use plan::Plan;
use plan::NUMBER_OF_LEDS;

pub enum Message {
    LedsOn(Vec<usize>),
    LedsOff(Vec<usize>),
    AllLedsOn,
    AllLedsOff,
    LedsBlink(Vec<usize>),
}



pub fn start(receive_channel: Receiver<Message>) -> () {
    let led_controller_sender = set_up_led_controller();
    let mut new_plan = Plan::AllOff;

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
                new_plan.all_on();
            }
            Message::AllLedsOff => {
                new_plan.all_off();
            }
            Message::LedsBlink(leds) => {
                new_plan.all_off();
                for led in leds {
                    if led < NUMBER_OF_LEDS {
                        new_plan.add(led);
                    }
                }
                todo!();
            }
        };
        match led_controller_sender.send(new_plan.clone()) {
            Err(_e) => { panic!("[Error] Unable to send new plan to LED Controller.") }
            _ => {}
        };
    }
    panic!("[Error] Lost all sending channels in Plan Generator.")
}

fn set_up_led_controller() -> mpsc::Sender<Plan> {
    let (send_end, receive_end) = mpsc::channel();
    thread::spawn(move || {
        led_controller::start(receive_end, );
    });
    send_end
}