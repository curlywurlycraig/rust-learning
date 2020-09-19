pub struct Guess {
    value: i32
}

#[derive(Debug)]
pub enum GuessErr {
    TooLow,
    TooHigh
}

impl Guess {
    pub fn new(value: i32) -> Result<Self, GuessErr> {
        if value < 0 {
            return Err(GuessErr::TooLow);
        } else if value > 100 {
            return Err(GuessErr::TooHigh);
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}