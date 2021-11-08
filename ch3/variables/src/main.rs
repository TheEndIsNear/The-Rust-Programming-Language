fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = x + 1;
    println!("The value of x is {}", x);

    let x = x * 2;
    println!("The value of x is {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is {}", y);

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("Another function");
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
