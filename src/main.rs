pub mod days {
    automod::dir!(pub "src/days");
}

fn main() -> anyhow::Result<()> {
    println!("Day 01\n------\n");
    println!("Part 1 answer: {}", days::day01::run()?.0);
    println!("Part 2 answer: {}", days::day01::run()?.1);
    Ok(())
}
