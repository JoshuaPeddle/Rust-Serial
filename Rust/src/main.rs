use std::time::{Duration, Instant};

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

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
            //std::thread::sleep(Duration::from_secs(1));
            let elapsed_time = now.elapsed();
            println!(" pps: {:?}", 1000000 / elapsed_time.as_micros());
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
}
