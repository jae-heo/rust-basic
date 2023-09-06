pub struct Point<T> {
    pub x : T,
    pub y : T,
}

impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }
}