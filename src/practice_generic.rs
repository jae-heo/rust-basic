/**
 * When using generic, the performance does not lower in running time. It is because the code will be duplicated in compile time.
 * for example, when we have code like 
 * pub struct Point<T> {
 *    pub x : T,
 *    pub y : T,
 * }
 * 
 * And in the code we use two kinds of point, Point<i32> and Point<f64>, the the code will be like..
 * pub struct Point<i32> {
 *    pub x : i32,
 *    pub y : i32,
 * }
 * pub struct Point<f64> {
 *    pub x : f64,
 *    pub y : f64,
 * }
 * 
 * Because this work is done in compile time, it does not effect runtime performance.
 */
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

