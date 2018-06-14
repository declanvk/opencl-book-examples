extern crate ocl;

use ocl::core::DeviceInfo;
use ocl::enums::DeviceInfo::*;
use ocl::{Device, Platform};

// const OPENCL_SRC_FILE: &'static str = "";

const DEVICE_INFO: [DeviceInfo; 14] = [
    MaxComputeUnits,
    MaxWorkItemDimensions,
    MaxWorkGroupSize,
    MaxWorkItemSizes,
    PreferredVectorWidthChar,
    PreferredVectorWidthShort,
    PreferredVectorWidthInt,
    PreferredVectorWidthLong,
    PreferredVectorWidthFloat,
    PreferredVectorWidthDouble,
    Extensions,
    LocalMemSize,
    GlobalMemSize,
    MaxConstantBufferSize,
];

fn main() -> ocl::Result<()> {
    for platform in Platform::list() {
        println!("Platform: {}", platform.name()?);

        println!("Devices: ");
        for device in Device::list_all(&platform)? {
            println!("{}", device.name()?);
            println!(
                "Max WG: {}, IS AVAIL: {}",
                device.max_wg_size()?,
                device.is_available()?
            );

            println!("Info:");
            for info_item in &DEVICE_INFO {
                println!("{:?}: {}", info_item, device.info(*info_item)?);
            }

            println!("\n");
        }

        println!("Extensions:");
        for extension in platform.extensions()?.iter() {
            println!("{}", extension);
        }

        println!("");
    }

    Ok(())
}
