const COLUMNS: usize = 32;
const ROWS: usize = 4;
pub const NUMBER_OF_LEDS: usize = ROWS * COLUMNS;
enum Column {
    Off,
    On([bool; COLUMNS])
}

pub enum Plan {
    AllOff,
    Plan([Column; ROWS]),
}

impl Plan {
    pub fn new() -> Plan {
        Plan::AllOff
    }
    pub fn all_off(&mut self) -> () {
        *self = Plan::AllOff;
    }
}