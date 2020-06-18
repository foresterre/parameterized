fn main() {
    println!(
        "part 1: {}",
        example_usage::go_to_apartment_level(include_str!("../input/day1")).unwrap()
    );

    println!(
        "part 2: {}",
        example_usage::entered_basement_at_instruction(include_str!("../input/day1")).unwrap()
    );
}
