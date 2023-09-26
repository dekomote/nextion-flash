use clap::Parser;

mod args;
use args::Args;
use nextion_flash::Connection;

fn main() {
    let args = Args::parse();

    let connection = match args.baud_rate {
        Some(b) => Connection::new(&args.serial_port, b).unwrap(),
        None => Connection::try_bauds(&args.serial_port).unwrap(),
    };

    connection
        .upload_file(&args.file_path, args.download_baud_rate)
        .unwrap();
}
