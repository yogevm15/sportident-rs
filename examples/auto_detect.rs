use serialport::{SerialPortInfo, SerialPortType};

use sportident::Reader;

const SPORTIDENT_VENDOR_ID: u16 = 4292;
const SPORTIDENT_READER_PRODUCT_ID: u16 = 32778;

fn list_serial_ports() -> Vec<SerialPortInfo> {
    let ports = serialport::available_ports().unwrap();
    ports
        .iter()
        .filter(|p| match p.port_type.clone() {
            SerialPortType::UsbPort(usb_info) => {
                usb_info.vid == SPORTIDENT_VENDOR_ID && usb_info.pid == SPORTIDENT_READER_PRODUCT_ID
            }
            _ => false,
        })
        .cloned()
        .collect()
}
#[tokio::main]
async fn main() {
    let available_ports = list_serial_ports();

    if available_ports.is_empty() {
        panic!("No SportIdent reader found. Please connect your reader and try again.");
    }

    println!("Connecting to reader at {}", &available_ports[0].port_name);
    match &available_ports[0].port_type {
        SerialPortType::UsbPort(usb_info) => {
            println!(
                "Vendor ID: {}, Product ID: {}, Manufacturer: {}, Product Description: {}",
                usb_info.vid,
                usb_info.pid,
                usb_info
                    .manufacturer
                    .clone()
                    .unwrap_or("No manufacturer found".to_string()),
                usb_info
                    .product
                    .clone()
                    .unwrap_or("No product description found".to_string())
            );
        }
        _ => unreachable!(),
    }

    let mut reader = Reader::connect(&available_ports[0].port_name)
        .await
        .expect("failed to connect");
    loop {
        let card_data = reader.poll_owner_data().await.expect("failed to poll card");
        
        println!("{:?}", card_data);
        
        reader
            .beep_until_card_removed()
            .await
            .expect("failed to beep");
    }
}
