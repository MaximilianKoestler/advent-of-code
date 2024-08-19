//! Advent of code 2015 day 20 part 2

fn create_sums_of_divisors(max: u32, limit: u32) -> Vec<u32> {
    let mut sums_of_divisors = vec![0; max as usize + 1];
    for i in 1..=max {
        for j in (1..limit).map(|k| k * i).take_while(|&j| j <= max) {
            sums_of_divisors[j as usize] += i;
        }
    }
    sums_of_divisors
}

fn sigma_prime_with_cache(n: u32, cache: &[u32]) -> u32 {
    cache[n as usize]
}

fn number_of_presents_with_cache(house_number: u32, cache: &[u32]) -> u32 {
    sigma_prime_with_cache(house_number, cache) * 11
}

#[allow(clippy::maybe_infinite_iter)]
fn find_house_with_at_least_x_presents(x: u32, cache: &[u32]) -> Option<u32> {
    (1..).find(|house_number| number_of_presents_with_cache(*house_number, cache) >= x)
}

fn main() {
    let input = 33_100_000;
    let cache = create_sums_of_divisors(input / 10, 50); // heuristic that works
    println!("Cache created");
    let solution = find_house_with_at_least_x_presents(input, &cache).unwrap();
    println!("Smallest house number: {solution}");
}
