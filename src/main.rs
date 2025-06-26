use hidapi::{HidApi, HidError};
use std::process::exit;

// TEMPer v1.4 (or any variant using the FM75 chip)
const VENDOR_ID: u16 = 0x0c45;
const PRODUCT_ID: u16 = 0x7401;

const TIMEOUT_MS: i32 = 1000;

fn main() -> Result<(), HidError> {
    let api = HidApi::new()?;

    let maybe_device_path = api
        .device_list()
        .find(|d| {
            d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID && d.interface_number() == 1
        })
        .map(|d| d.path());

    let device_path = if let Some(path) = maybe_device_path {
        path
    } else {
        eprintln!("Error: could not retrieve device path (is the device connected?)");
        exit(1);
    };

    let device = api.open_path(&device_path)?;

    // The first byte of data must contain the Report ID. For devices which only
    // support a single report, this must be set to 0x0. The remaining bytes
    // contain the report data. Therefore, calls to write() will always contain
    // one more byte than the report contains.
    let query: [u8; 9] = [0x00, 0x01, 0x80, 0x33, 0x01, 0x00, 0x00, 0x00, 0x00];
    let mut buffer: [u8; 8] = [0; 8];

    device.write(&query)?;
    device.read_timeout(&mut buffer, TIMEOUT_MS)?;
    println!("{}", convert_temper_buf(&buffer));

    Ok(())
}

// Please see https://github.com/edorfaus/TEMPered/wiki/FM75
fn convert_temper_buf(buf: &[u8; 8]) -> f32 {
    let raw_temp = ((buf[2] as i16) << 8) | (buf[3] as i16);
    raw_temp as f32 * (125.0 / 32000.0)
}
