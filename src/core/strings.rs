use crate::core::util::print_the_fuck_out;
use std::fmt::{Display, Formatter};

static mut LANGUAGE: Language = Language::En;

pub static UNKNOWN_COMMAND: Label = Label::new(
    "Unknown command",
    "Неизвестная команда",
);
pub static CONNECT_OR_DISCONNECT: Label = Label::new(
    "Connect or disconnect the Android device and press Enter",
    "Подключите или отключите Android устройство и нажмите Enter",
);
pub static NO_DEVICES_FOUND: Label = Label::new(
    "No devices found, try again",
    "Устройств не обнаружено, попробуйте снова",
);
pub static PLEASE_WAIT: Label = Label::new(
    "Please wait...",
    "Подождите...",
);
pub static SUCCESSFULLY: Label = Label::new(
    "Reconnect device and enjoy!",
    "Готово, переподключите устройство!",
);
pub static LINUX_ONLY: Label = Label::new(
    "Permission resolving is only applicable for Linux",
    "Исправление разрешений ADB применимо только для Linux",
);
pub static NO_ADB: Label = Label::new(
    "ADB wasn't recognized",
    "ADB не обнаружен",
);
pub static SELECT_DEVICE: Label = Label::new(
    "Select a device",
    "Выберите устройство",
);
pub static MEDIAS_NOT_FOUND: Label = Label::new(
    "Screenshots/casts were not found",
    "Скриншоты/записи не найдены",
);
pub static DESTINATION: Label = Label::new(
    "Destination: ",
    "Место назначения: ",
);
pub static SAVED: Label = Label::new(
    "Saved",
    "Сохранено",
);
pub static UNKNOWN_ERROR: Label = Label::new(
    "Unknown error",
    "Неизвестная ошибка",
);
pub static CANCEL: Label = Label::new(
    "Cancel",
    "Отмена",
);
pub static ERROR: Label = Label::new(
    "Error",
    "Ошибка",
);

pub enum Language {
    En,
    Ru,
}

impl Language {
    pub fn set_language(language: Language) {
        unsafe {
            LANGUAGE = language;
        }
    }
    fn get_language() -> &'static Language {
        unsafe { return &LANGUAGE }
    }
}

pub struct Label<'a> {
    variants: [&'a str; 2],
}

impl Label<'_> {
    pub const fn new<'a>(en: &'a str, ru: &'a str) -> Label<'a> {
        Label { variants: [en, ru] }
    }
    pub fn value(&self) -> &str {
        let index = match Language::get_language() {
            Language::En => 0,
            Language::Ru => 1,
        };
        return self.variants[index];
    }
    pub fn print(&self) {
        print!("{}", self);
        print_the_fuck_out();
    }
    pub fn println(&self) {
        println!("{}", self);
    }
}

impl Display for Label<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
