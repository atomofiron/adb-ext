use crate::core::util::print_the_fuck_out;
use std::fmt::{Display, Formatter};

static mut LANGUAGE: Language = Language::En;

pub const PLACEHOLDER: &str = "{}";

#[cfg(target_os = "linux")]
pub static NO_DEVICES_FOUND: Label = Label::new(
    "no devices without permissions found, try again",
    "устройств без разрешений не обнаружено, попробуйте снова",
);
#[cfg(target_os = "linux")]
pub static SUDO_EXPLANATION: Label = Label::new(
    "to fix the ADB permissions superuser is required",
    "чтобы получить доступ к ADB устройства(м), необходимы права суперпользователя",
);
#[cfg(target_os = "linux")]
pub static RECONNECT_DEVICES: Label = Label::new(
    "reconnect device(s) and enjoy!",
    "готово, переподключите устройство(а)!",
);
#[cfg(target_os = "linux")]
pub static WELL_DONE: Label = Label::new(
    "well done, enjoy!",
    "готово!",
);
#[cfg(not(target_os = "linux"))]
pub static LINUX_ONLY: Label = Label::new(
    "permission resolving is only applicable for Linux",
    "исправление разрешений ADB применимо только для Linux",
);
pub static INSTALLATION_SUCCEED: Label = Label::new(
    "installation succeed, run",
    "установка завершена, можете запустить",
);
pub static UPDATE_SUCCEED: Label = Label::new(
    "update succeed, run",
    "обновление завершена, можете запустить",
);
pub static SYMLINK_FAIL: Label = Label::new(
    "symlink creation failed: ",
    "ошибка при создании симлинка: ",
);
pub static NO_ADB: Label = Label::new(
    "aDB wasn't recognized",
    "aDB не обнаружен",
);
pub static NO_BUILD_TOOLS: Label = Label::new(
    "specify a path to the Android SDK or Build Tools in ",
    "укажите путь до Android SDK или Build Tools в ",
);
pub static NO_FILE: Label = Label::new(
    "no such file",
    "такого файла нет",
);
pub static NO_PATH: Label = Label::new(
    "no path specified",
    "путь не указан",
);
pub static SELECT_DEVICE: Label = Label::new(
    "select a device",
    "выберите устройство",
);
pub static MEDIAS_NOT_FOUND: Label = Label::new(
    "screenshots/casts were not found",
    "скриншоты/записи не найдены",
);
pub static NO_PACKAGE_NAME: Label = Label::new(
    "no package name passed",
    "не указано имя пакета",
);
pub static PRESS_ENTER_TO_STOP_REC: Label = Label::new(
    "press Enter to stop recording",
    "нажмите Enter, чтобы остановить запись",
);
pub static UNAUTHORIZED_BY_DEVICE: Label = Label::new(
    "unauthorized by the device",
    "на устройстве не дано разрешение",
);
#[cfg(unix)]
pub static HOWEVER_CONFIGURE: Label = Label::new(
    "... however, first of all to configure your current shell, run:",
    "... однако, для начала, чтобы настроить текущую сессию, запустите:",
);
#[cfg(windows)]
pub static HOWEVER_CONFIGURE: Label = Label::new(
    "... however, first of all add {} in the top of PATH and restart CMD",
    "... однако, для начала добавьте {} в самый верх PATH и перезапустить коммандрую строку",
);
pub static ADD_INTERPRETER: Label = Label::new(
    "add the interpreter into the hook file, for example #!/bin/bash or #!/bin/zsh",
    "добавьте интерпретатор в файл хука, например #!/bin/bash или #!/bin/zsh",
);
pub static INPUT_PARAMETERS_OR_EXIT: Label = Label::new(
    "input parameters or 'exit', 'quit', Ctrl-C to exit",
    "введите параметры или 'exit', 'quit', Ctrl-C, чтобы выйти",
);
pub static UNKNOWN: Label = Label::new(
    "unknown",
    "неизвестно",
);
pub static SAVED: Label = Label::new(
    "saved: {}",
    "сохранено: {}",
);
#[cfg(target_os = "linux")]
pub static UNKNOWN_ERROR: Label = Label::new(
    "unknown error",
    "неизвестная ошибка",
);
#[cfg(target_os = "linux")]
pub static SOMETHING_WRONG: Label = Label::new(
    "something went wrong(",
    "что-то пошло не так(",
);
pub static CANCEL: Label = Label::new(
    "cancel",
    "отмена",
);
pub static ERROR: Label = Label::new(
    "error",
    "ошибка",
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
        unsafe { &*&raw const LANGUAGE }
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
    pub fn println_formatted(&self, parts: &[&str]) {
        let mut value = self.value().to_string();
        let mut i = 0;
        while i < parts.len() {
            if let Some(pos) = value.find(PLACEHOLDER) {
                value.replace_range(pos..(pos + PLACEHOLDER.len()), parts[i]);
                i += 1;
            } else {
                break;
            }
        }
        println!("{value}");
    }
    pub fn eprintln(&self) {
        eprintln!("{}", self);
    }
}

impl Display for Label<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
