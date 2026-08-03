use crate::core::ext::{count_nbsp, fix_nbsp};

pub trait Trim {
    fn fix_nbsp_and_trim(&self) -> String;
}

impl Trim for Vec<u8> {

    fn fix_nbsp_and_trim(&self) -> String {
        match count_nbsp(&self) {
            0 => String::from_utf8_lossy(self).trim().to_string(),
            count => String::from_utf8_lossy(&fix_nbsp(self, count)).trim().to_string(),
        }
    }
}
