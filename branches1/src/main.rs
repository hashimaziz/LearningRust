fn main() {
    let number  = 6;

    if number < 5{
        println!("condition was true");
    } else if number != 0{
        println!("number does not equal 0")
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {}", number);

}
