use hidapi::{HidApi, HidError};
use std::process::exit;

// TEMPer v1.4 (or any variant using the FM75 chip)
const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x7401;

const TIMEOUT_MS: i32 = 1000;

/// Checks whether a specific USB device is present in the device list.
fn is_connected(vendor_id: u16, product_id: u16, api: &HidApi) -> bool {
    api.device_list()
        .any(|d| d.product_id() == product_id && d.vendor_id() == vendor_id)
}

fn main() -> Result<(), HidError> {
    let api = HidApi::new()?;
    if !is_connected(VENDOR_ID, PRODUCT_ID, &api) {
        eprintln!("Error: TEMPer device not connected");
        exit(1);
    }

    let maybe_device_path = api
        .device_list()
        .find(|d| {
            d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID && d.interface_number() == 1
        })
        .map(|d| d.path());

    let device_path = if let Some(path) = maybe_device_path {
        path
    } else {
        eprintln!("Error: could not retrieve device path");
        exit(1);
    };

    println!("Found TEMPer device at path: {:?}", device_path);
    let device = api.open_path(&device_path)?;

    let mut buffer: [u8; 8] = [0; 8];

    // The first byte of data must contain the Report ID. For devices which only
    // support a single report, this must be set to 0x0. The remaining bytes
    // contain the report data. Therefore, calls to write() will always contain
    // one more byte than the report contains.
    let query: [u8; 9] = [0x00, 0x01, 0x80, 0x33, 0x01, 0x00, 0x00, 0x00, 0x00];

    device.write(&query)?;
    device.read_timeout(&mut buffer, TIMEOUT_MS)?;
    println!("Buffer: {:?}", buffer);
    println!("Temp: {:?}", convert_temper_buf(&buffer));

    Ok(())
}

// Please see https://github.com/edorfaus/TEMPered/wiki/FM75
fn convert_temper_buf(buf: &[u8; 8]) -> f32 {
    let adj_temp = ((buf[2] as i32) << 8) + ((buf[3] as i32) & 0xFF);
    (adj_temp as f32) * (125.0 / 32000.0)
}
