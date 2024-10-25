use cpu_time::ProcessTime; // https://crates.io/crates/cpu-time
use std::time::Duration;
use algo_comp_exp_code::*;

pub enum Algorithm {
    SR1,
    SR2,
    SR3,
}

fn main() {
    const INPUT_MAX_X: i128 = 1000; // 実行時間が長い場合は 100 などに変更すると良い

    // 以下のコメントアウトを外して、各アルゴリズムの実験結果を表示
    print_experiment_result(INPUT_MAX_X, Algorithm::SR1);
    // print_experiment_result(INPUT_MAX_X, Algorithm::SR2);
    // print_experiment_result(INPUT_MAX_X, Algorithm::SR3);

    // 以下のコメントアウトを外して、各アルゴリズムの実験結果を表示（大きな入力値）
    // print_experiment_result(50 * INPUT_MAX_X, Algorithm::SR2);
    // print_experiment_result(50 * INPUT_MAX_X, Algorithm::SR3);
}

pub fn print_experiment_result(max_x: i128, select_algorithm: Algorithm) {
    let target_algorithm: fn(i128) -> i128 = match select_algorithm {
        Algorithm::SR1 => sr1,
        Algorithm::SR2 => sr2,
        Algorithm::SR3 => sr3,
    };

    let mut datas: Vec<(i128, u128)> = Vec::new();
    for x in 1i128..=max_x {
        let start_time: ProcessTime = ProcessTime::now(); // 計測開始
        let _res: i128 = target_algorithm(x);
        let duration: Duration = start_time.elapsed(); // 計測終了（計測結果）
        datas.push((x, duration.as_micros()));
    }

    let algorithm_name: &str = match select_algorithm {
        Algorithm::SR1 => "SR1",
        Algorithm::SR2 => "SR2",
        Algorithm::SR3 => "SR3",
    };

    println!("{} の実験結果", algorithm_name);
    println!("入力値 x の範囲: 1 ~ {}", max_x);
    for (x, time) in datas {
        println!("{}, {}", x, time);
    }
}