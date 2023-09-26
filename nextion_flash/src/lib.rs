use serial2::SerialPort;
use std::io::ErrorKind;
use std::time::Instant;
use std::{
    cell::RefCell,
    fs::File,
    io::{Error, Read},
    rc::Rc,
    thread,
    time::Duration,
};

const COMMAND_STOP: [u8; 3] = [0xff, 0xff, 0xff];
const COMMAND_CONNECT: [u8; 14] = [
    0x00, 0xff, 0xff, 0xff, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0xff, 0xff, 0xff,
];

pub struct Connection<'a> {
    pub port: Rc<RefCell<SerialPort>>,
    pub baud_rate: u32,
    pub device: &'a str,
}

impl<'a> Connection<'a> {
    pub fn try_bauds(device: &'a str) -> Result<Connection, Error> {
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
                    } else {
                        continue;
                    }
                }
            }
        }
        Err(Error::new(
            std::io::ErrorKind::NotConnected,
            "Error trying bauds.",
        ))
    }

    pub fn new(device: &'a str, baud_rate: u32) -> Result<Connection, Error> {
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
                Err(_) => break,
            };
        }
    }

    fn send_command(port: &SerialPort, command: &[u8]) -> Result<usize, Error> {
        port.write(command).and_then(|_| port.write(&COMMAND_STOP))
    }

    fn read_to_vec(port: &mut SerialPort, timeout: Duration) -> Result<Vec<u8>, Error> {
        let buf: &mut [u8; 1] = &mut [0; 1];
        let mut bytes_read: Vec<u8> = vec![];
        let start_t = Instant::now();

        port.set_read_timeout(timeout).unwrap();

        while start_t.elapsed() < timeout {
            match port.read_exact(buf) {
                Ok(()) => {
                    let read_value = buf[0];
                    if read_value != 0 {
                        bytes_read.push(read_value);
                        if read_value == 0x05 {
                            break;
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(bytes_read)
    }

    fn negotiate_upload_baud(&self, file_path: &str, baud_rate: u32) -> Result<SerialPort, Error> {
        let port = &mut self.port.borrow();

        unsafe {
            Connection::purge_read(&mut *self.port.as_ref().as_ptr());
        }

        let file = File::open(file_path).unwrap();
        let file_size = file.metadata().unwrap().len();

        let command = format!("whmi-wri {file_size},{baud_rate},0");

        Connection::send_command(port, b"").unwrap_or(0);
        Connection::send_command(port, command.as_bytes()).unwrap_or(0);
        thread::sleep(Duration::from_millis(50));
        let mut port = match serial2::SerialPort::open(self.device, baud_rate) {
            Ok(port) => port,
            Err(err) => {
                return Err(err);
            }
        };

        match Connection::read_to_vec(&mut port, Duration::from_millis(500)) {
            Ok(v) => {
                if !v.contains(&0x05) {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Didn't receive the right response",
                    ));
                };
                Ok(port)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn upload_file(&self, file_path: &str, baud_rate: u32) -> Result<(), Error> {
        let mut port = match self.negotiate_upload_baud(file_path, baud_rate) {
            Ok(port) => port,
            Err(err) => {
                return Err(err);
            }
        };
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(err) => return Err(err),
        };

        let file_size = file.metadata().unwrap().len();
        let mut chunk_counter = file_size / 4096 + 1;
        let last_chunk = file_size % 4096;

        let buf: &mut [u8; 1] = &mut [0; 1];
        let mut large_buf: Vec<u8> = vec![0; 4096];

        while chunk_counter > 0 {
            if chunk_counter == 1 {
                for _ in 0..last_chunk {
                    match file.read_exact(buf) {
                        Ok(()) => {
                            port.write(buf).unwrap();
                        }
                        Err(e) => return Err(e),
                    }
                }
            } else {
                match file.read_exact(&mut large_buf) {
                    Ok(_) => port.write(&large_buf).unwrap(),
                    Err(e) => return Err(e),
                };
            }

            thread::sleep(Duration::from_millis(4096000 / baud_rate as u64 + 10));

            match Connection::read_to_vec(&mut port, Duration::from_millis(500)) {
                Ok(v) => {
                    if !v.contains(&0x05) {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Didn't receive the right response",
                        ));
                    }
                }
                Err(e) => return Err(e),
            };

            chunk_counter -= 1;
        }

        Ok(())
    }

    fn connect(device: &'a str, baud_rate: u32) -> Result<Connection, Error> {
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

                port.set_read_timeout(Connection::get_connect_timeout(baud_rate))
                    .unwrap();
                port.read_exact(buf).and_then(|()| {
                    if buf == b"comok" {
                        Ok(Connection {
                            port: Rc::new(RefCell::new(port)),
                            baud_rate,
                            device,
                        })
                    } else {
                        Err(Error::new(
                            std::io::ErrorKind::NotConnected,
                            "Connection not established. Try and restart the device",
                        ))
                    }
                })
            }
            Err(err) => Err(err),
        }
    }
}
