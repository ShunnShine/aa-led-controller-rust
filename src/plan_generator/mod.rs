use std::sync::mpsc::Receiver;


pub enum Message {
    LedsOn(Vec<u8>),
    LedsOff(Vec<u8>),
    AllLedsOn,
    AllLedsOff,
    LedsBlink(Vec<u8>),
}

pub fn start(receive_channel: Receiver<Message>) -> () {
    const NUMBER_OF_LEDS: u8 = 4 * 32;
    let current_plan;
    let new_plan;

    

    for message in receive_channel {
        match message {
            Message::LedsOn(leds) => {
                
            }
            Message::LedsOff(leds) => {

            }
            Message::AllLedsOn => {

            }
            Message::AllLedsOff => {

            }
            Message::LedsBlink(leds) => {

            }
        }
    }
    panic!("[Error] Lost all sending channels in Plan Generator.")
}