use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::{
    input::{input, InputEvent, KeyEvent},
    screen::RawScreen,
};

use usbserial;

fn main() {
    user_input();
    echo();
}

fn user_input() {
    fn handle_input_event(event: InputEvent) -> &'static str {
        match event {
            // We have an event
            InputEvent::Keyboard(KeyEvent::Enter) => {
                println!("Enter Pressed");
                return "";
            }
            InputEvent::Keyboard(KeyEvent::Esc) => {
                println!("Program closing ...");
                return "break";
            }
            InputEvent::Mouse(event) => {
                /* Mouse event */
                println!("Mouse event, {:?}", event);
                return "";
            }
            _ => {
                /* Other events */
                println!("Other event, {:?}", event);
                return "";
            }
        }
    }
    const TICK_DELAY: Duration = Duration::from_millis(50);

    let ports = usbserial::get_connected_serial_ports(true);

    let mut port = serialport::new("COM6", 57600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    println!("Opened port");
    println!("Press 'ESC' to quit.");
    // Enable raw mode and keep the `_raw` around otherwise the raw mode will be disabled
    let _raw = RawScreen::into_raw_mode();

    // Create an input from our screen
    let input = input();
    input
        .enable_mouse_mode()
        .expect("Failed to enable mouse capture");

    // Create an async reader
    let mut reader = input.read_async();

    loop {
        if let Some(input_event) = reader.next() {
            let handler_response = handle_input_event(input_event);
            if handler_response == "break" {
                break;
            }
        }
        thread::sleep(TICK_DELAY);
    }
}

fn echo() {
    let ports = usbserial::get_connected_serial_ports(true);

    let mut port = serialport::new("COM6", 57600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
    println!("Opened port");

    let output = "T".as_bytes();
    port.write(output).expect("Write failed!");

    loop {
        let now = Instant::now();

        let current_bytes = port.bytes_to_read().unwrap();
        //println!("{:?}", current_bytes);
        if current_bytes < 1 {
            continue;
        } else {
            let mut serial_buf: Vec<u8> = vec![0; 10];
            port.read(serial_buf.as_mut_slice()).expect("Read failed!");
            //println!("{:?}", serial_buf);
            println!("{}", String::from_utf8_lossy(&serial_buf));
            port.write(output).expect("Write failed!");
            std::thread::sleep(Duration::from_millis(100));
            let elapsed_time = now.elapsed();
            println!(" pps: {:?}", 1000000 / elapsed_time.as_micros());
        }
    }
}

// std::thread::sleep(Duration::from_secs(1));
// port.write(output).expect("Write failed!");
// std::thread::sleep(Duration::from_secs(1));
// port.write(output).expect("Write failed!");
// std::thread::sleep(Duration::from_secs(1));
// println!("Wrote to port");
// // wait
// std::thread::sleep(Duration::from_secs(1));
// let mut serial_buf: Vec<u8> = vec![0; 32];
// port.read(serial_buf.as_mut_slice()).expect("Found no data!");
// println!("{}", String::from_utf8_lossy(&serial_buf));
