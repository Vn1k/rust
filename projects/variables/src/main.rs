// fn main() {
//     let x = 5; //imutable because in rust type is imutable. So you cant change the value in future
//     println!("Your number is {x}");
//     x = 6;
//     println!("Your number is {x}");
// }

fn main() {
    let mut x = 5; //when you assign `mut` in the variable, it means the value can be changed in future
    println!("Your number is {x}");
    x = 6;
    println!("Your number is {x}");
}

