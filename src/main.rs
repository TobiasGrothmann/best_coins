use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

fn get_coins_used_traces(coins: &[usize], target: usize) -> Vec<Option<usize>> {
    let mut num_coins = vec![None; target + 1];
    let mut used_coins = vec![None; target + 1];

    num_coins[0] = Some(0);

    for amount in 1..=target {
        for &coin in coins {
            if coin > amount {
                continue;
            }

            if let Some(change_before) = num_coins[amount - coin] {
                if num_coins[amount].is_none() || change_before + 1 < num_coins[amount].unwrap() {
                    num_coins[amount] = Some(change_before + 1);
                    used_coins[amount] = Some(coin);
                }
            }
        }
    }
    return used_coins;
}

struct Change {
    pub amount: usize,
    pub coins_used: Option<Vec<usize>>,
}
fn reconstruct_all_change(used_coins: &[Option<usize>]) -> Vec<Change> {
    used_coins
        .iter()
        .enumerate()
        .map(|(target_amount, coin)| {
            if coin.is_none() {
                return Change {
                    amount: target_amount,
                    coins_used: None,
                };
            }

            // Reconstruct coin path
            let mut coins_used = Vec::new();
            let mut current = target_amount;
            while let Some(coin) = used_coins[current] {
                coins_used.push(coin);
                current -= coin;
            }
            return Change {
                amount: target_amount,
                coins_used: Some(coins_used),
            };
        })
        .collect()
}

fn get_average_number_of_coins(coins: &[usize], max_target: usize) -> Option<f64> {
    let coins_used_traces = get_coins_used_traces(&coins, max_target);
    let all_changes = reconstruct_all_change(&coins_used_traces);

    let all_valid = all_changes
        .iter()
        .skip(1)
        .filter(|change| change.coins_used.is_none())
        .collect::<Vec<_>>()
        .len()
        == 0;
    if !all_valid {
        return None;
    }

    let total_coins_used: usize = all_changes
        .iter()
        .skip(1)
        .filter_map(|change| change.coins_used.as_ref())
        .map(|coins| coins.len())
        .sum();
    return Some(total_coins_used as f64 / (max_target as f64));
}

// fn generate_coin_variations(max_coin_value: usize) -> Vec<Vec<usize>> {
//     let mut coin_variations = Vec::new();
//     for a in 0..=max_coin_value {
//         for b in a..=max_coin_value {
//             for c in b..=max_coin_value {
//                 coin_variations.push(vec![a, b, c]);
//             }
//         }
//     }
//     coin_variations
// }

fn generate_coin_variations(num_coins: usize, max_coin_value: usize) -> Vec<Vec<usize>> {
    fn helper(
        num_coins: usize,
        max_coin_value: usize,
        start: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if current.len() == num_coins {
            result.push(current.clone());
            return;
        }
        for value in start..=max_coin_value {
            current.push(value);
            helper(num_coins, max_coin_value, value, current, result);
            current.pop();
        }
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    helper(num_coins, max_coin_value, 1, &mut current, &mut result);
    result
}

fn main() {
    let max_target = 499;
    let coin_variations = generate_coin_variations(4, max_target / 2 + 1);

    let coin_averages: Vec<(&Vec<usize>, Option<f64>)> = coin_variations
        .iter()
        .enumerate()
        .par_bridge()
        .map(|(i, coins)| {
            if i % 100000 == 0 {
                let progress_percentage = (i as f64 / coin_variations.len() as f64) * 100.0;
                println!("Progress: {:.2}%", progress_percentage);
            }
            let average = get_average_number_of_coins(&coins, max_target);
            return (coins, average);
        })
        .collect();

    let mut best_coins: Vec<usize> = Vec::new();
    let mut best_average = f64::MAX;
    for (coins, average) in coin_averages {
        if let Some(avg) = average {
            if avg < best_average {
                best_average = avg;
                best_coins = coins.clone();
            }
        }
    }
    println!("Best coins: {:?}", best_coins);
    println!("Best average: {:?}", best_average);
}
