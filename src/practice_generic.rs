pub struct Point<T> {
    pub x : T,
    pub y : T,
}

impl<T> Point<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }
}

pub struct Point2<T, U> {
    pub x : T,
    pub y : U
}

impl<T, U> Point2<T, U> {
    pub fn mix_point<T2, U2>(self, other : Point2<T2, U2>) -> Point2<T, U2> {
        Point2 {x : self.x, y : other.y}
    }
}