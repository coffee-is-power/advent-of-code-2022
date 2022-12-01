pub fn read_input(day: u8, real: bool) -> String {
    std::fs::read_to_string(format!(
        "input/day-{day}{}.txt",
        if real { "" } else { "-example" }
    ))
    .unwrap()
}
