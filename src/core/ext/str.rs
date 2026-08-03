use std::path::PathBuf;

pub trait StrExt {
    fn last_index(&self) -> usize;
    fn index_of(&self, c: char) -> Option<usize>;
    fn last_index_of(&self, c: char) -> Option<usize>;
    fn file_name(&self) -> String;
    fn path(&self) -> PathBuf;
}

fn inner_index_of(value: &str, c: char, rev: bool) -> Option<usize> {
    let mut index = if rev { value.last_index() } else { 0 };
    match rev {
        true => for char in value.chars().rev() {
            if char == c { return Some(index) }
            index -= 1;
        },
        false => for char in value.chars() {
            if char == c { return Some(index) }
            index += 1;
        },
    }
    return None
}

impl StrExt for str {

    fn last_index(&self) -> usize {
        self.len() - 1
    }

    fn index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, false)
    }

    fn last_index_of(&self, c: char) -> Option<usize> {
        inner_index_of(self, c, true)
    }

    fn file_name(&self) -> String {
        let offset = self
            .last_index_of('/')
            .map(|it| it + 1)
            .unwrap_or(0);
        return self.to_string()[offset..].to_string()
    }

    fn path(&self) -> PathBuf {
        PathBuf::from(self)
    }
}
