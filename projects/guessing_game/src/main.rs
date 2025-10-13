use std::io; //standard (std) library input output (io)
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    /*
    `thread_rng()` is a random number generator
    `.gen_range(start..=end)` is range between for the random number to generate
    */

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
    `.read_line()` meminjam variabel `guess` untuk diisi dengan input pengguna.
    `&` mengreferensikan ke argument pertama yang dibuat (mut guess) supaya tidak harus membuat data yang sama di memory yang sama terus menerus.
    */

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    /* 
    pada rust memungkinkan untuk menggunakan variable yang sama (guess) untuk didefined lagi ini disebut shadowing. 
    biasanya digunakan untuk mengubah value dari tipe A ke tipe lain.

    `.trim` untuk menghilangkan sesuatu yang tidak perlu seperti `\n` pada linux atau pada windows `\r\n`.
    `.parse` metode pada tipe string yang digunakan untuk mengconvert ke tipe lain yang dimana tipe lain didefinisikan diawal `let guess: u32`. 
    Dan `parse` hanya bisa mengconvert sesuatu yang bisa di convert ke numbers. Jika string mengandung emote itu tidak akan bisa di convert ke numbers, 
    karna its not possible dan akan masuk ke `.expect`
    */

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"), //arm 1
        Ordering::Greater => println!("Too big"), //arm 2
        Ordering::Equal => println!("Correct"), //arm 3
    }

    /* 
    `.cmp`untuk membandingkan dua value guess vs secret number dalam kasus ini
    `match` untuk pengambilan keputusan setelah `.cmp` mendapatkan 2 perbandingan value, yang keputusannya tersebut ada di setiap arm yang telah didefined.

    `Pattern => Expression/code` <- this is arm pattern
    */

    println!("You guessed: {guess}");

    /*
    {guess} menampilkan value dari variable untuk ditampilkan, dalam typescript seperti ${guess} dengan backticks
    */
}
