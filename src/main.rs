use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("bienvenue au juste prix");
    let secret_number = rand::thread_rng().gen_range(1,102);
    let mut count = 0;
    let mut essais_restant = 5;
    let count = loop{
        count += 1;
        let mut prix = String::new();

        println!("donnez un prix");

        io::stdin().read_line(&mut prix)
            .expect("Je n'est pas réussi a lire la ligne");

        let prix: u32 = match prix.trim().parse::<u32>(){
            Ok(_t) => _t,
            Err(_e) => {
                println!("Veuillez entrer un nombre svp ");
                continue;
            }
        };

        match prix.cmp(&secret_number){
            Ordering::Equal => {
                println!("Vous avez gagnez");
                println!("Vous avez réussi au bous de {} essai(s) ", count);
                break;
            }
            Ordering::Greater => println!("c'est moins"),
            Ordering::Less => println!("c'est plus "),
        }

        essais_restant = essais_restant - 1;
        if essais_restant <= 0 {
            println!("vous avez perdu");
            return;
        }
    };
}