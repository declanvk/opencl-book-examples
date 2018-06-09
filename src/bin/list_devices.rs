extern crate ocl;

use ocl::{Device, Platform};

// const OPENCL_SRC_FILE: &'static str = "";

fn main() -> ocl::Result<()> {
    for platform in Platform::list() {
        println!("{}:", platform.name()?);

        for device in Device::list_all(&platform)? {
            println!("{}", device.name()?);
        }

        println!("");
    }

    Ok(())
}
