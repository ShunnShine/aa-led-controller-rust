use std::{sync::mpsc::{Receiver, self}, time::Duration};
use rust_gpiozero::OutputDevice;
use super::plan::{Plan, Column};

struct LedController{
    ser: OutputDevice,
    rsk: OutputDevice,
    sck: OutputDevice,
    channels: [OutputDevice; 4],
}

impl LedController {
    fn new() -> LedController{
        let ch1 = OutputDevice::new(26);
        let ch2 = OutputDevice::new(19);
        let ch3 = OutputDevice::new(13);
        let ch4 = OutputDevice::new(6);
        let channels = [ch1, ch2, ch3, ch4];
        LedController{
            ser: OutputDevice::new(4),
            rsk: OutputDevice::new(3),
            sck: OutputDevice::new(2),
            channels,
        }
    }
    fn turn_all_off(&mut self) {
        for channel in &mut self.channels {
            channel.off();
        }
    }
    fn push_physical_buffer(&mut self, led: &bool) {
        if *led {self.ser.on();} else {self.ser.off();}
        self.sck.on();
        self.sck.off();
    }
    fn use_buffer(&mut self) {
        self.rsk.off();
        self.rsk.on();
    }
}

pub fn start(receiving_channel: Receiver<Plan>) {
    let mut controller = LedController::new();
    controller.turn_all_off();
    let mut plan = Plan::AllOff;
    loop {
        plan = {
            match plan {
                Plan::AllOff | Plan::OneColumn{..} => {
                    receiving_channel.recv().unwrap()
                }
                Plan::MultipleColumns{..} => {
                    match receiving_channel.try_recv() {
                        Ok(plan) => plan,
                        Err(error) => {
                            match error {
                                mpsc::TryRecvError::Empty => plan,
                                mpsc::TryRecvError::Disconnected => panic!("[Error] Lost all sending channels in Led Controller."),
                            }
                        }
                    }
                }
            }
        };
        match plan {
            Plan::AllOff => {
                controller.turn_all_off();
            },
            Plan::OneColumn{ref column, column_index} => {
                match column { 
                    Column::Off => { panic!("[Error] Off Column found in OneColumn Plan."); },
                    Column::On{ref leds, num_leds_on:_} => {
                        turn_on_channel(&mut controller, column_index, leds);
                    }
                }
            },
            Plan::MultipleColumns{ref columns, num_columns_on:_}=> {
                for (i, column) in columns.iter().enumerate() {
                    match column {
                        Column::Off => {},
                        Column::On{ref leds, num_leds_on:_} => {
                            turn_on_channel(& mut controller, i, leds);
                            std::thread::sleep(Duration::from_millis(4));
                            controller.channels[i].off();
                        }
                    }
                }
            }
        }
    }
}


fn turn_on_channel(controller: & mut LedController, channel_index: usize, leds: &[bool; 32]) {
    for led in leds.iter().rev() {
        controller.push_physical_buffer(led);
    }
    controller.use_buffer();
    controller.channels[channel_index].on();
}