//! Advent of code 2015 day 20 part 1

fn create_sums_of_divisors(max: u32) -> Vec<u32> {
    let mut sums_of_divisors = vec![0; max as usize + 1];
    for i in 1..=max {
        for j in (i..=max).step_by(i as usize) {
            sums_of_divisors[j as usize] += i;
        }
    }
    sums_of_divisors
}

fn sigma(n: u32) -> u32 {
    (1..=n).filter(|i| n % i == 0).sum()
}

fn sigma_with_cache(n: u32, cache: &[u32]) -> u32 {
    cache[n as usize]
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn sigma_upper_bound(n: u32) -> u32 {
    // according to Ramanujan, sigma(n) < e^γ * n * log(log(n)) for large enough n
    // (assuming Riemann hypothesis)
    const EULER_MASCHERONI: f64 = 0.577_215_664_901_532_9;
    let n = f64::from(n);
    (n * EULER_MASCHERONI * n.ln() * n.ln()).ceil() as u32
}

fn number_of_presents(house_number: u32) -> u32 {
    sigma(house_number) * 10
}

fn number_of_presents_with_cache(house_number: u32, cache: &[u32]) -> u32 {
    sigma_with_cache(house_number, cache) * 10
}

#[allow(clippy::maybe_infinite_iter)]
fn find_house_with_at_least_x_presents(x: u32, cache: &[u32]) -> Option<u32> {
    // Robin's theorem: n is large enough if n > 5040
    if x > 5040 {
        let sum_of_divisors = x / 10;

        // find a a lower bound for the house number using the upper bound of sigma
        // i.e. find the smallest n such that sigma_upper_bound(n) >= sum_of_divisors
        // using binary search
        let mut left = 1;
        let mut right = x;
        while left < right {
            let mid = (left + right) / 2;
            if sigma_upper_bound(mid) >= sum_of_divisors {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        (left..).find(|house_number| number_of_presents_with_cache(*house_number, cache) >= x)
    } else {
        (1..).find(|house_number| number_of_presents(*house_number) >= x)
    }
}

fn main() {
    let input = 33_100_000;
    let cache = create_sums_of_divisors(input / 10); // heuristic that works
    println!("Cache created");
    let solution = find_house_with_at_least_x_presents(input, &cache).unwrap();
    println!("Smallest house number: {solution}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_presents() {
        assert_eq!(number_of_presents(1), 10);
        assert_eq!(number_of_presents(2), 30);
        assert_eq!(number_of_presents(3), 40);
        assert_eq!(number_of_presents(4), 70);
        assert_eq!(number_of_presents(5), 60);
        assert_eq!(number_of_presents(6), 120);
        assert_eq!(number_of_presents(7), 80);
        assert_eq!(number_of_presents(8), 150);
        assert_eq!(number_of_presents(9), 130);
    }

    #[test]
    fn test_number_of_presents_with_cache() {
        let cache = create_sums_of_divisors(100);
        assert_eq!(number_of_presents_with_cache(1, &cache), 10);
        assert_eq!(number_of_presents_with_cache(2, &cache), 30);
        assert_eq!(number_of_presents_with_cache(3, &cache), 40);
        assert_eq!(number_of_presents_with_cache(4, &cache), 70);
        assert_eq!(number_of_presents_with_cache(5, &cache), 60);
        assert_eq!(number_of_presents_with_cache(6, &cache), 120);
        assert_eq!(number_of_presents_with_cache(7, &cache), 80);
        assert_eq!(number_of_presents_with_cache(8, &cache), 150);
        assert_eq!(number_of_presents_with_cache(9, &cache), 130);
    }
}
