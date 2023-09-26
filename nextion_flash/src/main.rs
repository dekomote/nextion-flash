use clap::Parser;

mod args;
use args::Args;
use nextion_flash::NextionConnection;

fn main() {
    let args = Args::parse();

    let connection = match args.baud_rate {
        Some(b) => NextionConnection::new(&args.serial_port, b).unwrap(),
        None => NextionConnection::try_bauds(&args.serial_port).unwrap(),
    };

    connection
        .upload_file(&args.file_path, args.download_baud_rate)
        .unwrap();
}
