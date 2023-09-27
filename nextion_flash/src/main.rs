use clap::Parser;
use termion::color;

mod args;
use args::Args;
use nextion_flash::NextionConnection;

fn main() {
    let args = Args::parse();

    let connection = match args.baud_rate {
        Some(baud_rate) => match NextionConnection::new(&args.serial_port, baud_rate) {
            Ok(conn) => {
                println!(
                    "{}Established connection on {}@{}",
                    color::Fg(color::Green),
                    args.serial_port,
                    baud_rate,
                );
                conn
            }
            Err(e) => {
                println!("{}Error connecting on {}@{}. Check connection to the display, or omit the baud rate to try other baud rates. ({e})", color::Fg(color::Red), args.serial_port, baud_rate);
                return;
            }
        },
        None => match NextionConnection::try_bauds(&args.serial_port) {
            Ok(conn) => {
                println!(
                    "{}Established connection on {}@{}",
                    color::Fg(color::Green),
                    args.serial_port,
                    conn.baud_rate,
                );
                conn
            }
            Err(e) => {
                println!(
                    "{}Error connecting to {}. Check connection to the display. ({e})",
                    color::Fg(color::Red),
                    args.serial_port
                );
                return;
            }
        },
    };

    match connection.upload_file(&args.file_path, args.download_baud_rate) {
        Ok(()) => {
            println!("{}Display flashed successfully!", color::Fg(color::Green));
        }
        Err(e) => {
            println!("{}Error uploading file. ({e})", color::Fg(color::Red));
        }
    }
}
