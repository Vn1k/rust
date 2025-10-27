// fn without_mutable() {
//     let x = 5; //imutable because in rust type is imutable. So you cant change the value in future
//     println!("Your number is {x}");
//     x = 6;
//     println!("Your number is {x}");
// }

fn with_mutable() {
    let mut x = 5; //when you assign `mut` in the variable, it means the value can be changed in future
    println!("Your number is {x}");
    x = 6;
    println!("Your number is {x}");
}

fn shadowing() {
    let x = 5; //meng declare x = 5

    let x = x + 1; //menggunakan trik shadowing dimana variable x di panggil lagi dan ditambah kan 1;

    { //ketika masuk kedalam scope baru value dari x yang sudah ditambahkan dijadikan referensi untuk variable shadowing pada x 
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    //dan ketika keluar dari scope value x tetap akan memakai variable x yang di shadowing sebelum scope karna itu variable terakhir yang di declare
    println!("The value of x in outter scope is: {x}");
}

fn main () {
    with_mutable();
    // without_mutable();
    shadowing();
}
