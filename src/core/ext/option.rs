
pub trait OptionExt<T> {
    fn take_some_if<F>(self, f: F) -> Option<T> where F: FnOnce(&T) -> bool;
    fn if_some<F>(self, f: F) -> Option<T> where F: FnOnce(&T);
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce();
    fn or_err<E>(self, e: E) -> Result<T, E>;
    fn or_else_try<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn take_some_if<F>(self, f: F) -> Option<T> where F: FnOnce(&T) -> bool {
        match &self {
            None => self,
            Some(value) if f(value) => self,
            _ => None,
        }
    }
    fn if_some<F>(self, f: F) -> Option<T> where F: FnOnce(&T) {
        match &self {
            Some(value) => f(value),
            None => (),
        }
        return self
    }
    fn if_none<F>(self, f: F) -> Option<T> where F: FnOnce() {
        match &self {
            None => f(),
            Some(_) => (),
        }
        return self
    }
    fn or_err<E>(self, e: E) -> Result<T, E> {
        match self {
            Some(t) => Ok(t),
            None => Err(e),
        }
    }
    fn or_else_try<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T> {
        match self {
            None => f(),
            s => s,
        }
    }
}
