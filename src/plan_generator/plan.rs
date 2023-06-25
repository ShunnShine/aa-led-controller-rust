const NUMBER_OF_COLUMNS: usize = 4;
const NUMBER_OF_ROWS: usize = 32;
pub const NUMBER_OF_LEDS: usize = NUMBER_OF_ROWS * NUMBER_OF_COLUMNS;

#[derive(Copy, Clone)]
pub enum Column {
    Off,
    On{leds:[bool; NUMBER_OF_ROWS], num_leds_on: usize},
}
type ColumnArray = [Column; NUMBER_OF_COLUMNS];

const COLUMN_ALL_ON: Column = Column::On{leds: [true; NUMBER_OF_ROWS], num_leds_on: NUMBER_OF_ROWS};
const ALL_COLUMNS_ON: ColumnArray = [COLUMN_ALL_ON; NUMBER_OF_COLUMNS];

#[derive(Copy, Clone)]
pub enum Plan {
    AllOff,
    OneColumn{column: Column, column_index: usize},
    MultipleColumns{columns: ColumnArray, num_columns_on: usize},
}


impl Plan {
    pub fn all_off(&mut self) -> () {
        *self = Plan::AllOff;
    }
    pub fn all_on(&mut self) -> () {
        *self = Plan::MultipleColumns{columns: ALL_COLUMNS_ON, num_columns_on: NUMBER_OF_LEDS};
    }
    pub fn add(&mut self, led_number: usize) -> () {
        if led_number >= NUMBER_OF_LEDS {
            println!("[Debug] Invalid led number.");
            return;
        }
        let row_index = led_number % NUMBER_OF_ROWS;
        let column_index = led_number / NUMBER_OF_ROWS; 
        match self {
            Plan::AllOff => {
                let mut new_column = [false; NUMBER_OF_ROWS];
                new_column[row_index] = true;
                *self = Plan::OneColumn{column: Column::On{leds: new_column, num_leds_on: 1}, column_index};
            },
            Plan::OneColumn{ref mut column, column_index: existing_column_index} => {
                if *existing_column_index == column_index {
                    match column {
                        Column::Off => { panic!("[Error] Off Column found in OneColumn Plan."); },
                        Column::On{ref mut leds, ref mut num_leds_on} => {
                            if leds[row_index] == false {
                                leds[row_index] = true;
                                *num_leds_on += 1;
                            }
                        }
                    }
                } else {
                    let mut new_column = [false; NUMBER_OF_ROWS];
                    new_column[row_index] = true;
                    let mut column_array = [Column::Off; NUMBER_OF_COLUMNS];
                    column_array[*existing_column_index] = *column;
                    column_array[column_index] = Column::On{leds: new_column, num_leds_on: 1};
                    *self = Plan::MultipleColumns{columns: column_array, num_columns_on: 2};
                }
                
            }
            Plan::MultipleColumns{ref mut columns, ref mut num_columns_on } => {
                match columns[column_index] {
                    Column::Off => {
                        let mut new_column = [false; NUMBER_OF_ROWS];
                        new_column[row_index] = true;
                        columns[column_index] = Column::On{leds: new_column, num_leds_on: 1};
                        *num_columns_on += 1;
                    }
                    Column::On{ref mut leds, ref mut num_leds_on} => {
                        if leds[row_index] == false {
                            leds[row_index] = true;
                            *num_leds_on += 1;
                        }
                    }
                }
                
            }
        }
    }
    pub fn remove(&mut self, led_number: usize) -> () {
        if led_number >= NUMBER_OF_LEDS {
            println!("[Debug] Invalid led number.");
            return;
        }
        let row_index = led_number % NUMBER_OF_ROWS;
        let column_index = led_number / NUMBER_OF_ROWS;
        match self {
            Plan::MultipleColumns{ref mut columns, ref mut num_columns_on } => {
                if let Column::On{ ref mut leds, ref mut num_leds_on } = columns[column_index] {
                    if leds[row_index] == true {
                        leds[row_index] = false;
                        *num_leds_on -= 1;
                        if *num_leds_on == 0 {
                            columns[column_index] = Column::Off;
                            *num_columns_on -= 1;
                            if *num_columns_on == 1 {
                                for (i, column) in columns.iter().enumerate() {
                                    if let Column::On{..} = column {
                                        *self = Plan::OneColumn{column: *column, column_index: i};
                                        break;
                                    }
                                }
                            }
                            else if *num_columns_on == 0 {
                                *self = Plan::AllOff;
                            }
                        }
                    }
                }
            },
            Plan::OneColumn{ref mut column, column_index: existing_column_index} => {
                match column {
                    Column::Off => { panic!("[Error] Off Column found in OneColumn Plan."); },
                    Column::On{ref mut leds, ref mut num_leds_on} => {
                        if *existing_column_index == column_index {
                            if leds[row_index] == true {
                                leds[row_index] = false;
                                *num_leds_on -= 1;
                                if *num_leds_on == 0 {
                                    *self = Plan::AllOff;
                                }
                            }
                        }
                    }
                }
            },
            Plan::AllOff => { }
        }
    }
}