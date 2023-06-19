use std::sync::mpsc::Receiver;

pub enum Message {
    LedsOn(Vec<u8>),
    LedsOff(Vec<u8>),
    AllLedsOn,
    AllLedsOff,
    LedsBlink(Vec<u8>),
}

pub fn start(receiveChannel: Receiver<Message>) -> () {

}