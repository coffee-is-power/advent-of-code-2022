pub fn read_input(day: u8) -> String {
    std::fs::read_to_string("input/day-{day}.txt").unwrap()
}