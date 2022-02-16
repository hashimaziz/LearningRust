fn main() {
    println!("Hello, world!");

    another_function(5);
    print_label_measurement(6, 'm');

    let y = {
        let a = 3;
        a + 1
    };
    println!("the value of y is: {}", y);

    let b = five();
    println!("The value of b is: {}", b);

    let c = plus_one(5);
    println!("The value of c is: {}", c);
}
fn another_function(x: i32){
    println!("The value of x is: {}", x);
}
fn print_label_measurement(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}
fn five() -> i32{ //define a return value using arrow and type of return
    5
}
fn plus_one(x : i32) -> i32{
    x + 1
}