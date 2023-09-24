use std::rc::Rc;
use clap::Parser;

mod logging;

const COMMAND_START: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
const COMMAND_STOP: [u8; 3] = [0xff, 0xff, 0xff];
const COMMAND_CONNECT: [u8; 7] = [0x63, 0x6F, 0x6E, 0x6E, 0x65, 0x63, 0x74];


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// E.g. /dev/ttyUSB0
    ///
    /// On Windows, use something like "COM1".
    /// For COM ports above COM9, you need to use the win32 device namespace,
    /// for example "\\.\COM10" (or "\\\\.\\COM10" with string escaping).
    #[arg()]
    serial_port: String,

    /// Path to the file to be flashed
    #[arg()]
    file_path: String,

    /// Initial baud rate.
    /// Faster baud rates will be tried if the setting is set.
    #[arg(short, long, default_value_t = 9600)]
    baud_rate: u32,

    /// Do not try faster baud negotiations.
    #[arg(short, long, default_value_t = false)]
    skip_faster_baud_trial: bool,
}


fn main() {
    let args = Args::parse();

    logging::init_logger();

    let port = match serial2::SerialPort::open(args.serial_port, args.baud_rate) {
        Ok(port) => port,
        Err(err) => {
            println!("Error opening serial port {}", err);
            return;
        }
    };

    let port = Rc::new(port);

    // Lets negotiate
    
}
