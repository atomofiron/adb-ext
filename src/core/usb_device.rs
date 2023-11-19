use crate::core::ext::Split;
use crate::core::r#const::{COLON, SPACE};

pub struct UsbDevice {
    pub vendor_id: String,
    pub product_id: String,
    pub description: String,
}

impl Clone for UsbDevice {
    fn clone(&self) -> Self {
        UsbDevice {
            vendor_id: self.vendor_id.clone(),
            product_id: self.product_id.clone(),
            description: self.description.clone(),
        }
    }
}

impl UsbDevice {
    pub fn from(usb_device: &String) -> UsbDevice {
        let parts = usb_device.splitn_to_vec(7, SPACE);
        let ids = parts.get(5).unwrap().split_to_vec(COLON);
        UsbDevice {
            vendor_id: ids.get(0).unwrap().clone(),
            product_id: ids.get(1).unwrap().clone(),
            description: parts.get(6).unwrap_or(&"".to_string()).clone(),
        }
    }
}
