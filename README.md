# nextion-flash

Program for flashing nextion displays with tft files via Serial connection.

The program was born from the desire to learn Rust better, and several
broken SD cards trying to flash the nextion display.

## Install

Download the latest binary from [here](https://github.com/dekomote/nextion-flash/releases/)
for your operating system and execute it.

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
