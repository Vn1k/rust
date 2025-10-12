use std::io; //standard (std) library input output (io)
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    /*
    thread_rng() is a random number generator
    .gen_range(start..=end) is range between for the random number to generate
    */

    println!("The secret number is: {secret_number}");

    println!("Input your guess: ");

    let mut guess = String::new(); //mutable and new empty string for guess variable

    /*
    let is to defined variable
    mut means mutable, which means the value can be changed. without it, the value can't be change (immutable).
    */

    io::stdin() //user input init
        .read_line(&mut guess) // Membaca input dan menyimpannya ke dalam `guess` melalui referensi mutable.
        .expect("Failed to read line"); // Menangani error jika pembacaan gagal.
        
    /*
    .read_line() meminjam variabel `guess` untuk diisi dengan input pengguna.
    `&` mengreferensikan ke argument pertama yang dibuat (mut guess) supaya tidak harus membuat data yang sama di memory yang sama terus menerus.
    */
    println!("You guessed: {guess}");

    /*
    {guess} menampilkan value dari variable untuk ditampilkan, dalam typescript seperti ${guess} dengan backticks
    */
}
