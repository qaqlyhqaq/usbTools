#![feature(random)]
use std::io::Write;
use uuid::Uuid;

fn main() -> anyhow::Result<()> {
    let random1 = Uuid::new_v4();
    if let Ok(is_exists) = std::fs::exists("info"){
        if is_exists.eq(&false){
            std::fs::create_dir("info")?;
        }
    }
    let mut buffer = std::fs::File::create(format!("info/{}.txt",random1))?;
    for dev in nusb::list_devices()? {

        ///filter usb device
        if 0x0483 == dev.vendor_id() && 0x070B == dev.product_id() {
            //no anything
        }else {
            continue;
        }

        ///output information into file buffer
        write!(buffer, "{:#?}\n", dev)?;
    }
    Ok(())
}
