use rust_gpiozero::OutputDevice;

fn main() {
    let mut ser  =  OutputDevice::new(4);
    let mut rsk  =  OutputDevice::new(3);
    let mut sck  =  OutputDevice::new(2);
    let mut ch1  = OutputDevice::new(26);
    let mut ch2  = OutputDevice::new(19);
    let mut ch3  = OutputDevice::new(13);
    let mut ch4  =  OutputDevice::new(6);

    
    let stdin = std::io::stdin();
    
    loop {
        println!("1\t2\t3\t4\tRSK\tSER\tSCK");
        println!("{}\t{}\t{}\t{}\t{}\t{}\t{}", ch1.value(), ch2.value(), ch3.value(), ch4.value(), rsk.value(), ser.value(), sck.value());
        
        let mut input = String::new();
        match stdin.read_line(&mut input){
            Ok(..) => {}
            Err(..) => {break;}
        }
        let command = input.trim();
        if command == "1"{
            ch1.toggle();
        }
        else if command == "2"{
            ch2.toggle();
        }
        else if command == "3"{
            ch3.toggle();
        }
        else if command == "4"{
            ch4.toggle();
        }
        else if command == "r"{
            rsk.toggle();
        }
        else if command == "ser"{
            ser.toggle();
        }
        else if command == "sck"{
            sck.toggle();
        }
        else if command == "exit"{
            break;
        }
    }
}

    
