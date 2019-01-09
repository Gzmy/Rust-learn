extern crate rand;

use std::io; //引入标准io库
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("GUess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mutable
        //创建一个String空字符串

        //stdin返回一个指向终端标准输入的句柄
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); //调用Result值

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            //Ordering like enum
            Ordering::Less     => println!("Too samll"),
            Ordering::Greater  => println!("Too big"),
            Ordering::Equal    => {
                println!("You win");
                break;
            }
        }
    }
}
