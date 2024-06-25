// struct User {
//     name: String,
//     age: u32,
//     active: bool,
// }

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}
fn main() {
    //structs    similar to  objects in javascript
    // let name = String::from("Imtiyaz");

    // let user = User {
    //     name: name,
    //     age: 25,
    //     active: true,
    // };

    // println!(
    //     "{} is {} years old and is {}",
    //     user.name, user.age, user.active
    // )

    let rect: Rect = Rect {
        width: 10,
        height: 20,
    };
    println!("The area of rectangle is {}", rect.area())
}
