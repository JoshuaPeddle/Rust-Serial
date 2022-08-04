use serialport::SerialPortInfo;

pub fn get_connected_serial_ports(verbose: bool) -> Vec<SerialPortInfo> {
    let ports = serialport::available_ports().expect("No ports found!");

    if verbose {
        for p in &ports {
            println!("{}", p.port_name);
        }
    }

    ports
}
