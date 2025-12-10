use std::fs;
use std::ops::{Add, BitAnd, Not};

fn round_int_to_even_ceil<T>(n: T) -> T
where T: Copy + Add<Output = T> + BitAnd<T, Output = T> + From<u8>
{
    let one: T = 1.into();
    n + (n & one)
}

fn round_int_to_even_floor<T>(n: T) -> T
where T: Copy + Not<Output = T> + BitAnd<T, Output = T> + From<u8>
{
    let one: T = 1.into();
    n & !one
}

pub fn solve() {
    let input = fs::read_to_string("./src/day_02.input.txt").unwrap();
    let mut total_id_count: u64 = 0;
    let mut invalid_id_sum: u64 = 0;

    for  line in input.split(",") {
        let range: Vec<&str> = line.split("-").collect();
        println!("\nRange {}", line);
        let range_lower = range.get(0).unwrap();
        let range_upper = range.get(1).unwrap();
        let limit_lower = range_lower.parse::<u64>().unwrap();
        let limit_upper = range_upper.parse::<u64>().unwrap();
        let full_range = limit_upper - limit_lower;
        
        let digits_lower = range_lower.len();
        let digits_upper = range_upper.len();
        println!("X  :Line: {}-{}", digits_lower, digits_upper);

        let digits_lower_rnd = round_int_to_even_ceil(digits_lower);
        let digits_upper_rnd = round_int_to_even_floor(digits_upper);
        println!("RND:Line: {}-{}", digits_lower_rnd, digits_upper_rnd);

        // when both digits were equal and odd before, the range is fully valid
        println!("Total unfiltered in range: {}", full_range);
        total_id_count += full_range;
        // all inclusive range
        total_id_count += 1;
        if digits_lower_rnd > digits_upper_rnd {
            println!("Skip Checks for {}", full_range);
            continue
        }

        // even loop
        for digits in (digits_lower_rnd..(digits_upper_rnd + 1)).step_by(2) {
            println!("Digits: {}", digits);
            let digits_half = (digits / 2) as u32;
            let digits_pow = 10_u64.pow(digits_half);
            let mut first_half_lower_limit = digits_pow / 10;
            let mut first_half: u64 = digits_pow - 1;
            if digits == digits_lower {
                first_half_lower_limit = limit_lower / 10_u64.pow(digits_half);
                let second_lower_half = limit_lower - first_half_lower_limit * digits_pow;
                if second_lower_half > first_half_lower_limit {
                    first_half_lower_limit += 1;
                }
            }
            if digits == digits_upper {
                first_half = limit_upper / 10_u64.pow(digits_half);
                let second_upper_half = limit_upper - first_half * digits_pow;
                if second_upper_half < first_half {
                    first_half -= 1;
                }
            }
            println!("Lower Half {}, Upper Half {}", first_half_lower_limit, first_half);
            // gartehag
            let mut num_patterns = 1_u64;
            num_patterns += first_half;
            num_patterns -= first_half_lower_limit;
            // we know know the amount of patterns:
            println!("Num patterns {}", num_patterns);
            total_id_count -= num_patterns;

            while first_half_lower_limit <= first_half {
                let pattern_num = first_half + first_half * digits_pow;
                println!("Adding {}", pattern_num);
                invalid_id_sum += pattern_num;
                first_half -= 1;
            }
        }
    }
    println!("TOTAL: {}", total_id_count);
    println!("INVALID_SUM: {}", invalid_id_sum)
}
