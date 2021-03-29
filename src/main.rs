use hw3::{ Circle, Triangle, Square, get_area};

fn main() {
    let a = Circle {
        r: 2.0
    };
    println!("the area of this circle is: {}", get_area(a));

    let b = Triangle {
        b: 4.0,
        h: 3.0,
    };
    println!("the area of this triangle is: {}", get_area(b));

    let c = Square {
        a: 5.0,
    };
    println!("the area of this square is: {}", get_area(c));
}
