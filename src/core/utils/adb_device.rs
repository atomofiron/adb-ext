pub struct AdbDevice {
    pub serial: String,
    pub model: String,
    pub ok: bool,
    pub unauthorized: bool,
    pub no_permissions: bool,
}

pub trait AdbDeviceVec {
    fn get_unique_model_name(&self, device: &AdbDevice) -> String;
}

impl AdbDeviceVec for Vec<AdbDevice> {
    fn get_unique_model_name(&self, device: &AdbDevice) -> String {
        let mut count = 0;
        for d in self {
            if d.model == device.model {
                count += 1;
            }
        }
        return if count > 1 {
            format!("{} ({})", device.model, device.serial)
        } else {
            device.model.clone()
        }
    }
}
