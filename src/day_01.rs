pub fn solve() -> Result<(), std::io::Error> {
    println!("SOLVING DAY 1");
    let data: String = std::fs::read_to_string("./src/day_01.input.txt")?;
    println!("datalength{}", data.len());
    const MODULO_SIZE: i16 = 100;
    let mut current_solution: i16 = 50;
    let mut zeroes_count: u16 = 0;
    let mut clicks_count: i16 = 0;
    for line in data.lines() {
        let mut input = line.to_string();
        let direction = input.remove(0);
        let sign: i16 = if direction == 'R' { 1 } else { -1 };
        let movement: i16 = input.parse::<i16>().unwrap() * sign;
        // we have to correct for a false positive movement_q which will be one too high if it goes from 0 to LEFT
        if current_solution == 0 && direction == 'L' {
            clicks_count -= 1;
        }
        // println!("Adding {} and {}", current_solution, movement_i16);
        // println!("Previous Sol: {}", current_solution);
        current_solution += movement;
        let movement_q = current_solution.div_euclid(MODULO_SIZE);
        // println!("Movement {}\n MovementQ {}\n CurrentSol {}", movement, movement_q, current_solution);
        current_solution = current_solution.strict_rem_euclid(MODULO_SIZE);
        // println!("{}",sign);
        if current_solution == 0 {
            zeroes_count += 1;
            // println!("ZERO HIT!");
            // we have to correct for the case where we turned left and perfectly landed on the 0, which will cause q to be 0
            if direction == 'L' {
                // println!("Left to 0");
                clicks_count += 1;
            }
        }
        // println!("MOVE Q {} MOVE Q2 {}", movement_q, movement_q_2);
        clicks_count += movement_q.abs();
        // println!(" Result {} \n", current_solution);
    }
    println!("ZEROES COUNT: {}", zeroes_count);
    println!("CLICKS COUNT: {}", clicks_count);
    Ok(())
}
