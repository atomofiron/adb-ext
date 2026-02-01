
pub enum StartMode {
    Adb,
    AdbExt,
    Unknown,
}

impl StartMode {
    pub fn adb(&self) -> bool {
        matches!(self, StartMode::Adb)
    }
}
