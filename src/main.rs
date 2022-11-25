use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyUSB0";
    let baud_rate: u32 = 115200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        /* successfully opened the serial port */
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        /* failed to open the serial port */
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        }
    }
}
