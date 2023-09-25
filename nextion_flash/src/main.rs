use std::{rc::Rc, time::Duration, io::{Error, Read}, thread, cell::RefCell, borrow::BorrowMut};
use clap::Parser;
use serial2::SerialPort;

mod logging;

const COMMAND_STOP: [u8; 3] = [0xff, 0xff, 0xff];
const COMMAND_CONNECT: [u8; 14] = [0x00, 0xff, 0xff, 0xff, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0xff, 0xff, 0xff];


pub struct Connection<'a>{
    pub port: Rc<RefCell<SerialPort>>,
    pub baud_rate: u32,
    pub device: & 'a str,
}

impl<'a> Connection<'a> {

    pub fn try_bauds(device: & 'a str) ->  Result<Connection, Error> {
        let mut baud_rates = Vec::from(serial2::COMMON_BAUD_RATES);
        baud_rates.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        for (pos, baud_rate) in baud_rates.iter().enumerate() {
            log::info!("Trying baud {baud_rate}");
            match Connection::connect(device, *baud_rate) {
                Ok(conn) => {
                    log::info!("Connected at baud {baud_rate}");
                    return Ok(conn);
                }
                Err(err) => {
                    if pos == baud_rates.len() {
                        return Err(err);
                    }
                    else {
                        continue;
                    }
                }
            }
        }
        Err(Error::new(std::io::ErrorKind::NotConnected, "Error trying bauds."))
    }

    pub fn new(device: & 'a str, baud_rate: u32) ->  Result<Connection, Error> {
        Connection::connect(device, baud_rate)
    }

    fn get_connect_timeout(baud_rate: u32) -> Duration {
        Duration::from_millis((30 + 11000000 / baud_rate).into())
    }

    fn purge_read(port: &mut SerialPort) {
        loop {
            let b: &mut [u8; 1] = &mut [0];
            match port.read_exact(b) {
                Ok(_) => {}
                Err(_) => break
            };
        }
    }
    
    fn connect(device: & 'a str, baud_rate: u32) -> Result<Connection, Error> {
        let mut port = match serial2::SerialPort::open(device, baud_rate) {
            Ok(port) => port,
            Err(err) => {
                return Err(err);
            }
        };

        Connection::purge_read(&mut port);

        match port.write_all(&COMMAND_CONNECT) {
            Ok(_) => {
                let buf: &mut [u8; 5] = &mut [0; 5];
                
                port.set_read_timeout(Connection::get_connect_timeout(baud_rate)).unwrap();
                port.read_exact(buf).and_then(|()| {
                    if buf == b"comok" {
                        return Ok(
                            Connection { port: Rc::new(RefCell::new(port)), baud_rate: baud_rate, device: device }
                        );
                    }
                    else {
                        return Err(
                            Error::new(std::io::ErrorKind::NotConnected, "Connection not established. Try and restart the device")
                        );
                    }
                })
            },
            Err(err) => {
                Err(err)
            }
        }
    }

}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// E.g. /dev/ttyUSB0
    ///
    /// On Windows, use something like "COM1".
    /// For COM ports above COM9, you need to use the win32 device namespace,
    /// for example "\\.\COM10" (or "\\\\.\\COM10" with string escaping).
    #[arg(default_value_t = String::from("/dev/ttyUSB1"))]
    serial_port: String,

    /// Path to the file to be flashed
    #[arg(default_value_t = String::from("1.txt"))]
    file_path: String,

    /// Initial baud rate.
    /// Optional, all baud rates will be tried if ommited.
    #[arg(short, long)]
    baud_rate: Option<u32>,

    /// File transfer baud rate.
    #[arg(short, long, default_value_t=115200)]
    download_baud_rate: u32,

}


fn main() {
    let args = Args::parse();

    logging::init_logger();

    let connection = match args.baud_rate {
        Some(b) => {
            Connection::new(&args.serial_port, b)
        },
        None => {
            Connection::try_bauds(&args.serial_port)
        }
    };
    
}
