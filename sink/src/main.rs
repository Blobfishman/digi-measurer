extern crate chrono;
extern crate csv;
extern crate serialport;

use chrono::prelude::*;
use csv::Writer;
use serialport::prelude::*;
use std::error::Error;
use std::time::Duration;
use std::thread;
use std::path::Path;
use std::string::String;

fn main() {
    //TODO get serial port
    //TODO get root_path from program arguments
    let mut session: MeasureSession = MeasureSession::setup(String::from(""), String::from(""));
    let mut cycle = 0;
    loop {
        let val = session.receive_data();
        session.insert_data(val);
        thread::sleep(Duration::from_millis(100));

        cycle += 1;
        if cycle >= 60000 {
            session.save_data();
            cycle = 0;
        }
    }
}

pub struct MeasureSession {
    port: serialport::posix::TTYPort,
    data: Vec<(DateTime<Local>, i32)>,
    device_name: String,
    root_path: String,
}

impl MeasureSession {
    pub fn receive_data(&mut self) -> i32 {
        let mut buffer: String;


        //TODO implement
        0
    }

    // insert new values only if it is different to the last one
    pub fn insert_data(&mut self, value: i32) {
        if self.data.last().is_none() || self.data.last().unwrap().1 != value {
            self.data.push((Local::now(), value));
        }
    }

    pub fn setup(path: String, serial_port: String) -> MeasureSession {
        let s = SerialPortSettings {
            baud_rate: 19200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(1),
        };
        MeasureSession {
            port: serialport::posix::TTYPort::open(Path::new(&serial_port), &s)
                .ok()
                .unwrap(),
            data: Vec::new(),
            device_name: String::from(""),
            root_path: path,
        }
    }

    pub fn save_data(&mut self) -> Result<String, Box<dyn Error>> {
        let path = self.root_path.clone() + &self.device_name;
        let mut wtr = Writer::from_path(&path)?;
        wtr.write_record(&["time", "value"])?;
        for (time, value) in &self.data {
            wtr.write_record(&[time.to_rfc3339(), value.to_string()])?;
        }
        wtr.flush()?;
        Ok(path)
    }
}
