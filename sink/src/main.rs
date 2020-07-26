extern crate chrono;
extern crate csv;
extern crate serialport;

use chrono::prelude::*;
use csv::Writer;
use serialport::prelude::*;
use std::env;
use std::error::Error;
use std::string::String;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("wrong program parameter!\nsink filename serial_port");
        return;
    }
    let mut session: MeasureSession = MeasureSession::setup(&args[1], &args[2]);
    let mut cycle = 0;
    loop {
        let val = session.receive_data();
        println!("val: {}", val);
        session.insert_data(val);
        thread::sleep(Duration::from_millis(100));

        cycle += 1;
        if cycle >= 60000 {
            session.save_data().ok();
            cycle = 0;
        }
    }
}

pub struct MeasureSession {
    port: Box<dyn serialport::SerialPort>,
    data: Vec<(DateTime<Local>, f32)>,
    device_name: String,
    root_path: String,
}

impl MeasureSession {
    pub fn receive_data(&mut self) -> f32 {
        let mut buffer = String::new();
        // do-while
        let _ = self.port.read_to_string(&mut buffer);
        while buffer.parse::<f32>().is_err() {
            eprintln!("reading from serial port failed");
            println!("val: {}", buffer);
            // ignore return value because serialport read_to_string returns an error even if it was successfull
            let _ = self.port.read_to_string(&mut buffer);
        }
        buffer.parse::<f32>().unwrap()
    }

    // insert new values only if it is different to the last one
    pub fn insert_data(&mut self, value: f32) {
        if self.data.last().is_none() || self.data.last().unwrap().1 != value {
            self.data.push((Local::now(), value));
        }
    }

    pub fn setup(path: &String, serial_port: &String) -> MeasureSession {
        let s = SerialPortSettings {
            baud_rate: 19200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(10),
        };
        MeasureSession {
            port: serialport::open_with_settings(serial_port, &s)
                .ok()
                .unwrap(),
            data: Vec::new(),
            device_name: String::from(""),
            root_path: path.clone(),
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
