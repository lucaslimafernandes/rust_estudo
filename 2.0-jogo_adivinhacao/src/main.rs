use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Digite seu palpite: ");

    let mut palpite = String::new();
    //cria uma var mútavel atualmente vinculada a uma nova 
    // instância vazia de uma String 

    //Outra forma std::io::stdin()
    io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada!");

    println!("Você disse: {}", palpite);

}
