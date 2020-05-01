pub trait TakeIf {
    type Inner;

    fn take_if<F: FnOnce(&Self::Inner) -> bool>(&mut self, predicate: F) -> Option<Self::Inner>;
}

impl<T> TakeIf for Option<T> {
    type Inner = T;

    fn take_if<F: FnOnce(&Self::Inner) -> bool>(&mut self, predicate: F) -> Option<Self::Inner> {
        if self.as_ref().map(predicate).unwrap_or(false) {
            self.take()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TakeIf;

    #[test]
    fn conditional_take() {
        let mut option = Some(5);
        assert_eq!(None, option.take_if(|_| false));
        assert_eq!(Some(5), option.take_if(|_| true));
        assert_eq!(None, option.take_if(|_| true));
    }
}
