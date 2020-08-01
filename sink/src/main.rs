extern crate chrono;
extern crate serialport;

use chrono::prelude::*;
use serialport::prelude::*;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
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
    loop {
        let val = session.receive_data();
        println!("val: {}", val);
        session.write_data(&val, Local::now());
        thread::sleep(Duration::from_millis(100));
    }
}

pub struct MeasureSession {
    port: Box<dyn serialport::SerialPort>,
    device_name: String,
    root_path: String,
    last_val: String,
}

impl MeasureSession {
    pub fn receive_data(&mut self) -> String {
        let mut buffer = String::new();
        // do-while
        let _ = self.port.read_to_string(&mut buffer);
        while buffer.trim().is_empty() {
            eprintln!("reading from serial port failed");
            // ignore return value because serialport read_to_string returns an error even if it was successfull
            let _ = self.port.read_to_string(&mut buffer);
        }
        eprintln!("return value");
        buffer
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
            device_name: String::from(""),
            root_path: path.clone(),
            last_val: String::from(""),
        }
    }

    pub fn write_data(&mut self, data: & String, time: DateTime<Local>) {
        println!("enter write data");
        if self.last_val.trim().eq(data.trim()) {
            println!("ignore value");
            return;
        }
        self.last_val = data.clone();
        let path: String = format!("{}{}", self.root_path, self.device_name);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

        if let Err(e) = writeln!(file, "{},{}\n", time.to_rfc3339(), data.trim()) {
            eprintln!("Couldn't write to file: {}", e);
        }
        file.flush().ok();
    }
}
