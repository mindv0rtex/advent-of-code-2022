pub mod days {
    automod::dir!(pub "src/days");
}

fn main() -> anyhow::Result<()> {
    println!("Day 01\n------");
    let solution = days::day01::run()?;
    println!("Part 1 answer: {}", solution.0);
    println!("Part 2 answer: {}\n", solution.1);

    println!("Day 02\n------");
    let solution = days::day02::run()?;
    println!("Part 1 answer: {}", solution.0);
    println!("Part 2 answer: {}\n", solution.1);

    println!("Day 03\n------");
    let solution = days::day03::run();
    println!("Part 1 answer: {}", solution.0);
    println!("Part 2 answer: {}", solution.1);

    Ok(())
}
