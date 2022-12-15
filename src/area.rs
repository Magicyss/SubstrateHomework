#[allow(dead_code)]
fn print_area<T: CalculateArea>(diagram: T) {
    print!("{}",diagram.get_area());
}
trait CalculateArea {
    fn get_area(&self) -> f64;
}

use std::f64::consts::PI;

struct Circle {
    r:f64
}

impl CalculateArea for Circle{
    fn get_area(&self) -> f64{
        return PI * self.r * self.r;
    }
}

struct Square {
    l:f64
}

impl CalculateArea for Square{
    fn get_area(&self) -> f64{
        return self.l * self.l;
    }
}

struct Triangle {
    a:f64,
    b:f64,
    c:f64
}

impl CalculateArea for Triangle{
    fn get_area(&self) -> f64{
        let p = (self.a + self.b + self.c) / 2.0;
        return (p*(p-self.a)*(p-self.b)*(p-self.c)).sqrt();
    }
}

#[test]
fn print_circle_area(){
    let circle = Circle{r:1.0};
    print_area(circle)
}

#[test]
fn print_square_area(){
    let square = Square{l:1.0};
    print_area(square)
}

#[test]
fn print_triangle_area(){
    let triangle = Triangle{a:3.0,b:4.0,c:5.0};
    print_area(triangle)
}