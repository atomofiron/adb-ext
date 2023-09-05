use crate::core::util::Split;

pub struct Device {
    pub vendor_id: String,
    pub product_id: String,
    pub description: String,
}

impl Clone for Device {
    fn clone(&self) -> Self {
        Device {
            vendor_id: self.vendor_id.clone(),
            product_id: self.product_id.clone(),
            description: self.description.clone(),
        }
    }
}

impl Device {
    pub fn from(usb_device: &String) -> Device {
        let parts = usb_device.splitn_to_vec(7, ' ');
        let ids = parts.get(5)
            .unwrap()
            .split_to_vec(':');
        Device {
            vendor_id: ids.get(0).unwrap().clone(),
            product_id: ids.get(1).unwrap().clone(),
            description: parts.get(6).unwrap().clone(),
        }
    }
}