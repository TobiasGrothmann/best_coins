use rayon::iter::{ParallelBridge, ParallelIterator};

fn get_coins_num_used(coins: &[usize], target: usize) -> Option<Vec<usize>> {
    let mut num_coins = vec![usize::MAX; target + 1];
    num_coins[0] = 0;

    for amount in 1..=target {
        for &coin in coins {
            if coin <= amount && num_coins[amount - coin] != usize::MAX {
                num_coins[amount] = num_coins[amount].min(num_coins[amount - coin] + 1);
            }
        }
        if num_coins[amount] == usize::MAX {
            return None;
        }
    }

    return Some(num_coins);
}

fn get_average_number_of_coins(coins: &[usize], max_target: usize) -> Option<f64> {
    let coins_used_traces = get_coins_num_used(&coins, max_target);
    if let Some(coins_used_traces) = coins_used_traces {
        let total_coins_used: usize = coins_used_traces.iter().skip(1).sum();
        return Some(total_coins_used as f64 / (max_target as f64));
    } else {
        return None;
    }
}

struct CoinVariations {
    num_coins: usize,
    max_coin_value: usize,
    stack: Vec<(Vec<usize>, usize)>,
}

impl CoinVariations {
    fn new(num_coins: usize, max_coin_value: usize) -> Self {
        Self {
            num_coins,
            max_coin_value,
            stack: vec![(vec![1], 2)],
        }
    }
}

impl Iterator for CoinVariations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current, next_value)) = self.stack.pop() {
            if current.len() == self.num_coins {
                return Some(current);
            }
            for val in (next_value..=self.max_coin_value).rev() {
                let mut next = current.clone();
                next.push(val);
                self.stack.push((next, val + 1));
            }
        }
        None
    }
}

fn main() {
    let max_target = 499;
    let num_coins = 4;
    let max_coin_value = max_target / 2 + 1;

    let coin_variations = CoinVariations::new(num_coins, max_coin_value);

    let (best_coins, best_average) = coin_variations
        .par_bridge()
        .map(|coins| {
            let avg = get_average_number_of_coins(&coins, max_target);
            (coins.clone(), avg)
        })
        .filter_map(|(coins, avg)| avg.map(|a| (coins, a)))
        .reduce(
            || (Vec::new(), f64::MAX),
            |acc, (coins, avg)| {
                if avg < acc.1 { (coins, avg) } else { acc }
            },
        );

    println!("Best coins: {:?}", best_coins);
    println!("Best average: {:?}", best_average);
}
