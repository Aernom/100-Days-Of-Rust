use std::{collections::HashMap, io::stdin};

fn main() {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Errore nella lettura della riga da terminale");

    let chars: Vec<_> = buffer.trim().chars().collect();

    for c in &chars {
        if *c < '2' || *c > '9' {
            println!("I caratteri inseriti devono essere numeri compresi tra 2 e 9 (inclusi)");
            return;
        }
    }

    let digit_map = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);

    let char_vectors: Vec<_> = chars.iter().map(|c| digit_map.get(c).unwrap()).collect();

    let combinations = get_combinations(&char_vectors[..]);

    combinations.iter().for_each(|comb| println!("{comb}"));
    println!("Sono state trovate {} combinazioni", combinations.len());
}

fn get_combinations(char_vectors: &[&Vec<char>]) -> Vec<String> {
    if char_vectors.len() <= 1 {
        match char_vectors.get(0) {
            Some(char_slice) => char_slice.iter().map(|c| c.to_string()).collect(),
            None => Vec::new(),
        }
    } else {
        let mut combinations: Vec<String> = Vec::new();

        let prefixes = char_vectors[0];

        for p in prefixes {
            combinations.extend(
                get_combinations(&char_vectors[1..])
                    .iter()
                    .map(|comb| p.to_string() + comb),
            );
        }

        combinations
    }
}
