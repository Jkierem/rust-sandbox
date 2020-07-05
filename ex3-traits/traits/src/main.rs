struct Point<T,U> {
    x: T,
    y: U,
}

trait PointLike<T,U> {
    fn x(&self) -> &T;
    fn y(&self) -> &U;
}

fn mult_point<A>(p0: A, p1: A) -> impl PointLike<i32,i32>
where 
    A: std::fmt::Display + PointLike<i32,i32> 
{
    println!("{} {}", p0, p1);
    let res = Point { 
        x: p0.x() * p1.x() , 
        y: p0.y() * p1.y() , 
    };
    res
}

impl<T: Copy> PointLike<T,T> for Point<T,T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

fn mult<U,T>(x: T , y: T) -> U 
where
    T: std::ops::Mul<T, Output=U>
{
    x * y
}

fn main() {
    let x = 5;
    {
        println!("{}",x);
        let y = 7;
    }
    x;
}