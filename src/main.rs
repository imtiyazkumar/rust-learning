fn main() {
    // let x = 8;
    // println!("x={}", x);
    // let x: i64 = 77777777744111177;
    // println!("Hello, world!");

    // let mut x = 10;
    // x = 15;

    //all variables are immutable by default

    // let is_male = true;
    // let is_above_18 = true;

    // if is_male {
    //     println!("you are a male");
    // } else {
    //     println!("you are not male");
    // }

    // if is_male && is_above_18 {
    //     println!("you are a legal male");
    // }

    // let greeting = String::from("hello Imtiyaz");

    // println!("{}", greeting);

    // for i in 0..10 {
    //     print!("{}", i)
    // }
    //prints 0123456789
    // let a = 10;
    // let b = 20;
    // let sum = do_sum(a, b);

    // println!("Sum of {} and {} is {}", a, b, sum);

    //ownership

    // let greeting = String::from("hello Imtiyaz");
    // println!("{}", greeting); //will work

    // let y = greeting;
    // println!("{}", greeting); //borrowed error
    // println!("{}", y);

    //Borrowing
    //& takes the reference only not the ownership

    // let greeting = String::from("hello Imtiyaz");

    // let y = &greeting;
    // println!("{}", greeting); //borrowed error
    // println!("{}", y);

    let mut s1 = String::from("Hello ");

    update_str(&mut s1);

    // s1.push_str("World");

    println!("{}", s1)

    //a variable can have only single mutable reference at a time
}

//return type of fn can't be inferred like in ts we need to mention it.
// fn do_sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }

fn update_str(s: &mut String) {
    s.push_str("World 11"); //cannot borrow `*s` as mutable, as it is behind a `&` reference
}
