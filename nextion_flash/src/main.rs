use clap::Parser;
use crossterm::style::Stylize;
use crossterm::style::{Color, SetForegroundColor};

mod args;
use args::Args;
use nextion_flash::NextionConnection;

fn main() {
    let args = Args::parse();

    let connection = match args.baud_rate {
        Some(baud_rate) => match NextionConnection::new(&args.serial_port, baud_rate) {
            Ok(conn) => {
                println!(
                    "{} {}@{}",
                    "Established connection on".green(),
                    args.serial_port.to_string().green().bold(),
                    baud_rate.to_string().green().bold(),
                );
                conn
            }
            Err(e) => {
                println!("{} {}@{}. {}  ({e})",
                    "Error connecting on".red(),
                    args.serial_port.to_string().bold().red(),
                    baud_rate.to_string().bold().red(),
                    "Check connection to the display, or omit the baud rate to try other baud rates.".red()
                );
                return;
            }
        },
        None => match NextionConnection::try_bauds(&args.serial_port) {
            Ok(conn) => {
                println!(
                    "{} {}@{}",
                    "Established connection on".green(),
                    args.serial_port.to_string().green().bold(),
                    conn.baud_rate.to_string().green().bold(),
                );
                conn
            }
            Err(e) => {
                println!(
                    "{}Error connecting to {}. Check connection to the display. ({e})",
                    SetForegroundColor(Color::Red),
                    args.serial_port
                );
                return;
            }
        },
    };

    match connection.upload_file(&args.file_path, args.download_baud_rate) {
        Ok(()) => {
            println!(
                "{}Display flashed successfully!",
                SetForegroundColor(Color::Green)
            );
        }
        Err(e) => {
            println!(
                "{}Error uploading file. ({e})",
                SetForegroundColor(Color::Red),
            );
        }
    }
}
