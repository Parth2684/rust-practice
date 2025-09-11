enum Shape {
    Circle(f64),
    Square(f64),
    Rectange(f64, f64)
}

fn main () {
    let circle_area = cal_area(Shape::Circle(54_f64));
    let square_area = cal_area(Shape::Square(654_f64));
    let rectangle_area = cal_area(Shape::Rectange(65_f64, 54_f64));

    println!("{}, {}, {}", circle_area, square_area, rectangle_area);
}

fn cal_area(shape: Shape) -> f64{
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectange(len,bre ) => len * bre
    }
    
}