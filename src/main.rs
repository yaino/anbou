mod button;

use button::Button;
use std::{error::Error, thread, time::Duration};

const GPIO_BUTTON: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let button = Button::new(GPIO_BUTTON)?;
    button.run();
    loop {
        thread::sleep(Duration::from_millis(100));
    }
}
