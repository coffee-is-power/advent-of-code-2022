use advent_of_code_2022::read_input;
fn main() {
    let input = read_input(1, true);
    let mut elfs_foods = input
        // Separar os elfs
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                // converte as calorias de strings para numeros
                .map(|elf| elf.parse::<u64>().unwrap())
                // Soma as calorias deste elfo
                .sum::<u64>()
        })
        // Transforma o iterator em um vetor
        .collect::<Vec<_>>();
    // Ordena as calorias totais para que os elfos com mais calorias fiquem no final da lista
    elfs_foods.sort();
    println!("{elfs_foods:#?}");
    // Printa elfo com mais calorias
    println!("Solução da Parte 1: {}", elfs_foods.last().unwrap());
    // Printa a soma das calorias dos 3 elfos com mais calorias
    println!(
        "Solução da Parte 2: {}",
        elfs_foods[elfs_foods.len() - 3..].iter().sum::<u64>()
    )
}
