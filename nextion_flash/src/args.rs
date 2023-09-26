use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// E.g. /dev/ttyUSB0
    ///
    /// On Windows, use something like "COM1".
    /// For COM ports above COM9, you need to use the win32 device namespace,
    /// for example "\\.\COM10" (or "\\\\.\\COM10" with string escaping).
    #[arg(default_value_t = String::from("/dev/ttyUSB0"))]
    pub serial_port: String,

    /// Path to the file to be flashed
    #[arg(default_value_t = String::from("/home/dejan/1.tft"))]
    pub file_path: String,

    /// Initial baud rate.
    /// Optional, all baud rates will be tried if ommited.
    #[arg(short, long)]
    pub baud_rate: Option<u32>,

    /// File transfer baud rate.
    #[arg(short, long, default_value_t = 2000000)]
    pub download_baud_rate: u32,
}
