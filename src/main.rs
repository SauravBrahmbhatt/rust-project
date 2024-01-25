fn main() {
    let s1: String = gives_ownership();
    println!("{s1}");

    let s2 = String::from("bye world");
    let s3 = takes_and_give_back_ownership(s2);

    println!("{s3}");
    // let s = String::from("hello world");
    // take_ownership(s);
    // // println!("the new string is {s}")

    // let x = 5;
    // take_ownership_number(x);
    // println!("the new number is {x}")
}

// fn take_ownership(some_string: String) {
//     println!("The string is {some_string}")
// }

// fn take_ownership_number(some_integer: i32) {
//     println!("the number is {some_integer}")
// }
fn gives_ownership() -> String {
    let some_string = String::from("hello world");
    some_string
}

fn takes_and_give_back_ownership(new_string: String) -> String {
    new_string
}
