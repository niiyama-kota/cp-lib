pub trait Add<T> {
    fn add(&mut self, v: T);
}

pub trait Size<T> {
    fn size(&self) -> usize;
}