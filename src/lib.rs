pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::fs;
pub fn load_file(name: &str) -> String {
    fs::read_to_string(String::from("test-vectors/") + name)
        .expect("Run cargo in the respository's root dir.")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {
        println!("Day 1 Task:");
        println!("[part 1]: {}", crate::day1::solve(2));
        println!("[part 2]: {}", crate::day1::solve(3));
        println!("");

        println!("Day 2 Task:");
        println!("[part 1]: {}", crate::day2::solve_v1());
        println!("[part 2]: {}", crate::day2::solve_v2());
        println!("");

        println!("Day 3 Task:");
        println!("[part 1]: {}", crate::day3::solve_v1());
        println!("[part 2]: {}", crate::day3::solve_v2());
        println!("");

        println!("Day 4 Task:");
        println!("[part 1]: {}", crate::day4::solve_v1());
        println!("[part 2]: {}", crate::day4::solve_v2());
        println!("");

        println!("Day 5 Task:");
        println!("[part 1]: {}", crate::day5::solve_v1());
        println!("[part 2]: {}", crate::day5::solve_v2());
        println!("");

        println!("Day 6 Task:");
        println!("[part 1]: {}", crate::day6::solve_v1());
        println!("[part 2]: {}", crate::day6::solve_v2());
        println!("");

        println!("Day 7 Task:");
        println!("[part 1]: {}", crate::day7::solve_v1());
        println!("[part 2]: {}", crate::day7::solve_v2());
        println!("");

        println!("Day 8 Task:");
        println!("[part 1]: {}", crate::day8::solve_v1());
        println!("[part 2]: {}", crate::day8::solve_v2());
        println!("");

        println!("Day 9 Task:");
        println!("[part 1]: {}", crate::day9::solve_v1());
        println!("[part 2]: {}", crate::day9::solve_v2());
        println!("");

        println!("Day 10 Task:");
        println!("[part 1]: {}", crate::day10::solve_v1());
        println!("[part 2]: {}", crate::day10::solve_v2());
        println!("");

        println!("Day 11 Task:");
        println!("[part 1]: {}", crate::day11::solve_v1());
        println!("[part 2]: {}", crate::day11::solve_v2());
        println!("");
    }
}
