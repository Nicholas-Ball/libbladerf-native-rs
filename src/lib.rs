use anyhow::Result;
use nusb::Interface;
use nusb::transfer::{ControlIn, ControlType, Recipient, RequestBuffer};

pub struct Device {
    pub(crate) vendor_id: u16,
    pub(crate) product_id: u16,

    pub(crate) interface: Option<Interface>,
}

pub fn list_devices() -> Result<Vec<Device>>{
    let mut devices = Vec::new();

    let list_devices = nusb::list_devices()?;

    for device in list_devices {

        if device.vendor_id() != 0x2CF0 {
            continue;
        }

        let device = Device {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),

            interface: None,
        };

        devices.push(device);
    }

    Ok(devices)
}

impl Device {
    pub async fn connect(&mut self) -> Result<()> {
        // Connect to the device
        for device in nusb::list_devices()? {
            if device.vendor_id() == self.vendor_id && device.product_id() == self.product_id {
                let int = device.open()?.claim_interface(0)?;
                self.interface = Some(int);
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("Device not found"))
    }

    pub async fn get_version(&mut self) -> Result<[u8;4]>{
        let mut to_return = [0;4];
        if let Some(int) = &mut self.interface {
            let buf = int.control_in(ControlIn{
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: 0,
                value: 0x00,
                index: 0x00,
                length: 4,
            }).await;

            for (i, byte) in buf.data.iter().enumerate() {
                to_return[i] = *byte;
            }

            Ok(to_return)
        }else {
            Err(anyhow::anyhow!("Device not connected"))
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        // Disconnect from the device
        self.interface = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.interface.is_some()
    }
}