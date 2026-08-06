use enum_is::EnumIs;

#[derive(EnumIs)]
pub enum StartMode {
    Adb,
    AdbExt,
    Unknown,
}
