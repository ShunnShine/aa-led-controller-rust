use std::thread;
use rust_gpiozero::OutputDevice;
pub struct LedController{
    

}

impl LedController {
    fn new() -> LedController{
        
        thread::spawn(|| {
            let ser: OutputDevice = OutputDevice::new(4);
            let rsk: OutputDevice = OutputDevice::new(3);
            let sck: OutputDevice = OutputDevice::new(2);
            let ch1: OutputDevice = OutputDevice::new(26);
            let ch2: OutputDevice = OutputDevice::new(19);
            let ch3: OutputDevice = OutputDevice::new(13);
            let ch4: OutputDevice = OutputDevice::new(6);


        });
        LedController{

        }
    }
    fn turn_all_on(&self){
        
    }

    fn turn_all_off(&self){

    }

    fn turn_on(&self, row: u32, col: u32){

    }

    fn turn_off(&self, row: u32, col: u32){
        
    }

    fn toggle(&self, row: u32, col: u32){
        
    }
}