use std::sync::mpsc::{Receiver, self};
use rust_gpiozero::OutputDevice;
use super::plan::Plan;

struct LedController{
    ser: OutputDevice,
    rsk: OutputDevice,
    sck: OutputDevice,
    ch1: OutputDevice,
    ch2: OutputDevice,
    ch3: OutputDevice,
    ch4: OutputDevice,
    current_plan: Plan,
}

impl LedController {
    fn new() -> LedController{
        LedController{
            ser: OutputDevice::new(4),
            rsk: OutputDevice::new(3),
            sck: OutputDevice::new(2),
            ch1: OutputDevice::new(26),
            ch2: OutputDevice::new(19),
            ch3: OutputDevice::new(13),
            ch4: OutputDevice::new(6),
            current_plan: Plan::new(),
        }
    }
}

pub fn start(receiving_channel: Receiver<Plan>) {
    let controller = LedController::new();
    loop {
        let new_plan = {
            if let Plan::AllOff = controller.current_plan {
                receiving_channel.recv().unwrap()
            } else {
                match receiving_channel.try_recv() {
                    Ok(plan) => plan,
                    Err(error) => {
                        match error {
                            mpsc::TryRecvError::Empty => controller.current_plan,
                            mpsc::TryRecvError::Disconnected => panic!("[Error] Lost all sending channels in Led Controller."),
                        }
                    }
                }
            }
        };
        match new_plan {
            Plan::AllOff => {
                controller.turn_all_off();
            }
            Plan::Plan(plan) => {
                for column in plan {
                    
                }
            }
        }
    }
}