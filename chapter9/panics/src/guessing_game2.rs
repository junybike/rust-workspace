pub struct Guess
{
    value: i32,
}

impl Guess
{
    pub fn new(value: i32) -> Guess // tests value to make sure it's between 1 and 100
    {
        if value < 1 || value > 100
        {
            panic!("Value must be between 1 and 100");
        }
        Guess {value}
    }
    pub fn value(&self) -> i32  // getter
    {
        self.value
    }
}