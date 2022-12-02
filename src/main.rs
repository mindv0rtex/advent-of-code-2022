pub mod days {
    automod::dir!(pub "src/days");
}

fn main() -> anyhow::Result<()> {
    println!("Day 01\n------\n");
    let solution = days::day01::run()?;
    println!("Part 1 answer: {}", solution.0);
    println!("Part 2 answer: {}", solution.1);
    Ok(())
}
