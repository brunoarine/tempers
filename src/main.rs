// fn main() {
//     for device in rusb::devices().unwrap().iter() {
//         let device_desc = device.device_descriptor().unwrap();

//         println!(
//             "Bus {:03} Device {:03} ID {}:{}",
//             device.bus_number(),
//             device.address(),
//             device_desc.vendor_id(),
//             device_desc.product_id()
//         );
//     }
// }

use rusb::{DeviceList, GlobalContext};

const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x7401;

// Document this function, AI!
fn is_connected(devices: DeviceList<GlobalContext>) -> bool {
    devices.iter().any(|d| -> bool {
        let descriptor_result = d.device_descriptor();
        match descriptor_result {
            Ok(descriptor) => {
                descriptor.vendor_id() == VENDOR_ID && descriptor.product_id() == PRODUCT_ID
            }
            Err(_) => false,
        }
    })
}

fn main() {
    let devices = rusb::devices().unwrap();
    println!("Device connected? {}", is_connected(devices));
}
