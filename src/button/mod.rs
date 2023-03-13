use chrono::Local;
use rppal::gpio::{Gpio, InputPin, Level};
use serde::Serialize;
use std::{error::Error, thread, time::Duration};
use ureq::json;

#[derive(Serialize, Debug)]
struct RequestBody {
    pub id: String,
    pub state: i8,
    pub distance: u32,
    pub detected_at: String,
}

pub struct Button {
    pin: InputPin,
    count: u8,
    status: Level,
}

impl Button {
    pub fn new(pin: u8) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pin: Gpio::new()?.get(pin)?.into_input(),
            count: 0,
            status: Level::High,
        })
    }

    pub fn run(mut self) {
        thread::spawn(move || loop {
            self.observe();
            thread::sleep(Duration::from_millis(10));
        });
    }

    pub fn observe(&mut self) {
        match self.pin.read() {
            Level::Low => {
                if self.status == Level::High {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::Low;
                        self.count = 0;
                        let url = "{APIの呼び出しURLをここに記述する}";
                        let body = RequestBody {
                            id: uuid::Uuid::new_v4().to_string(),
                            state: 1,
                            distance: 100,
                            detected_at: Local::now().to_rfc3339(),
                        };
                        ureq::put(url).send_json(json!(body)).unwrap();
                    }
                } else {
                    self.count = 0;
                }
            }
            Level::High => {
                if self.status == Level::Low {
                    self.count += 1;
                    if self.count > 3 {
                        self.status = Level::High;
                        self.count = 0;
                    }
                } else {
                    self.count = 0;
                }
            }
        }
    }
}
