# nextion-flash

A software for flashing tft files to nextion displays via a Serial connection.

The idea for this software was born from the desire to learn Rust better, and several
broken SD cards trying to flash the nextion display.

Use with caution! Although it hasn't happened yet after 100+ flashes, it might brick your device!    
Read the [Disclamer](#disclaimer)!


## Install

Download the latest binary from [here](https://github.com/dekomote/nextion-flash/releases/)
for your operating system and execute it.

Alternatively, clone the repository, and build it with cargo.
Assuming you have installed rust and rust tools:

``` bash
git clone git@github.com:dekomote/nextion-flash.git
cd nextion-flash/nextion_flash
cargo build --release
```
There should be a binary in `nextion_flash/target/release/nextion_flash[.exe]`

```

Usage: nextion_flash [OPTIONS] [SERIAL_PORT] [FILE_PATH]

Arguments:
  [SERIAL_PORT]
          E.g. /dev/ttyUSB0
          
          On Windows, use something like "COM1". For COM ports above COM9,
          you need to use the win32 device namespace, for example "\\.\COM10"
          (or "\\\\.\\COM10" with string escaping).

  [FILE_PATH]
          Path to the file to be flashed

Options:
  -b, --baud-rate <BAUD_RATE>
          Initial baud rate. Optional, all baud rates will be tried if ommited

  -d, --download-baud-rate <DOWNLOAD_BAUD_RATE>
          File transfer baud rate.
          [default: 2000000]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```



For more help

` nextion_flash --help.`

## Disclaimer

**I am not responsible for any damage caused to your hardware or software** while using this program or following the hardware-related instructions provided in this repository.

The software and guidance provided here are intended for educational and informational purposes only. It is your responsibility to exercise caution, follow best practices, and take appropriate precautions when working with hardware components.

Before attempting any hardware modifications or installations, it is strongly recommended that you:

1. **Read and understand the manufacturer's documentation** for your hardware.
2. **Back up your data** and important information to prevent data loss in case of unforeseen issues.
3. **Disconnect power sources and follow safety guidelines** when working with electrical components.
4. **Seek professional assistance** or consult with experts if you are unsure about any aspect of the hardware-related tasks.

By using this software or following any hardware-related instructions provided here, you acknowledge that you are doing so at your own risk, and the author of this repository shall not be held liable for any damage or loss incurred as a result of your actions.

Please proceed with caution and exercise diligence to protect your hardware and data.
