use std::collections::{HashSet, HashMap};
use rand::{seq::IteratorRandom};
use md5::{Md5, Digest};

fn fill_pancakes(num_pancakes: usize) -> Vec<char> {
    let dict = "abcdefghijklmnopqrstuvwxyz";
    let mut empty_pancakes = Vec::new();
    let mut seen_chars = HashSet::new();
    for _ in 0..num_pancakes {
        let mut random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        while seen_chars.contains(&random_char) {
            random_char = dict.chars().choose(&mut rand::thread_rng()).unwrap();
        }
        seen_chars.insert(random_char);
        empty_pancakes.push(random_char);
    }
    empty_pancakes
}

fn flip_pancakes(pancakes: &mut [char], index: usize) {
    if pancakes.len() < 2 { return; }
    pancakes[..index+1].reverse();
}

fn is_pancake_sorted(pancakes: &[char]) -> bool {
    for i in 1..pancakes.len() {
        if pancakes[i] < pancakes[i - 1] {
            return false;
        }
    }
    true
}

fn hash_permutation(permutation: &[char]) -> String {
    let input_string: String = permutation.iter().collect();
    let mut hasher = Md5::new();
    hasher.update(input_string.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn deep_first_search(permutation_inicial: &[char]) -> Vec<char> {
    let n = permutation_inicial.len();
    let mut visitados = HashSet::new();

    visitados.insert(hash_permutation(permutation_inicial));
    let mut stack = Vec::new();
    stack.push((permutation_inicial.to_owned(), 0, 0));
    let mut d = HashMap::<String, usize>::new();
    let mut p = HashMap::<String, Vec<char>>::new();

    let initial_permutation_hash = hash_permutation(permutation_inicial);
    d.insert(initial_permutation_hash.clone(), 0);
    p.insert(initial_permutation_hash.clone(), permutation_inicial.to_vec());

    while let Some((permutation, index, level)) = stack.pop() {
        if is_pancake_sorted(&permutation) {
            // si se encuentra la permutación ordenada, se detiene la búsqueda
            println!("NÚMERO DE NODOS EXPANDIDOS: {}", visitados.len());
            println!("NIVEL: {}", level);
            println!("NÚMERO DE NODOS EN LA COLA: {}", stack.len());
            println!("INDICE: {}", index);
            return permutation;
        }
        for i in 2..=n {
            let mut sucesor = permutation.clone();
            flip_pancakes(&mut sucesor, i - 1);
            let sucesor_hash = hash_permutation(&sucesor);
            if !visitados.contains(&sucesor_hash) {
                visitados.insert(sucesor_hash.clone());
                d.insert(sucesor_hash.clone(), d[&hash_permutation(&permutation)] + 1);
                p.insert(sucesor_hash.clone(), permutation.clone());
                stack.push((sucesor, i - 1, level + 1));
            }
        }
    }
    // si no se encuentra la permutación ordenada, devuelve la permutación inicial
    permutation_inicial.to_vec()
}

fn main() {
    let mut n = String::new();
    println!("Ingrese el numero de caracteres de pancakes: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let pancakes = fill_pancakes(n);
    println!("Pancakes generados: {:?}", pancakes);
    deep_first_search(&pancakes);
}
