use advent_of_code_2022::read_input;
fn main() {
    let input = read_input(1, true);
    let mut elfs_foods = input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|elf| elf.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();
    elfs_foods.sort();
    println!("{elfs_foods:#?}");
    println!("Solução da Parte 1: {}", elfs_foods.last().unwrap());
    println!(
        "Solução da Parte 2: {}",
        elfs_foods[elfs_foods.len() - 3..].iter().sum::<u64>()
    )
}
