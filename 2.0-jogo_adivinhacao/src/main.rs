extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    let numero_secreto = rand::thread_rng().gen_range(1, 3);

    loop {
        println!("Digite seu palpite: ");

        let mut palpite: String = String::new();
        //cria uma var mútavel atualmente vinculada a uma nova 
        // instância vazia de uma String 

        //Outra forma std::io::stdin()
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada!");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => { 
                println!("Você acertou!");
                break;
            }   
        }
        println!("Você disse: {}", palpite);

    }
}
