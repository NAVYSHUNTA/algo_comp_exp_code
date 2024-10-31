use cpu_time::ProcessTime; // https://crates.io/crates/cpu-time
use std::time::Duration;
use algo_comp_exp_code1::*;

// エントリーポイント
fn main() {
    const INPUT_MAX_X: i128 = 1000; // 実行時間が長い場合は 100 などに変更すると良い

    // 以下のコメントアウトを外して、各アルゴリズムの実験結果を表示
    print_experiment_result(INPUT_MAX_X, &Algorithm::SR1);
    // print_experiment_result(INPUT_MAX_X, &Algorithm::SR2);
    // print_experiment_result(INPUT_MAX_X, &Algorithm::SR3);

    // 以下のコメントアウトを外して、各アルゴリズムの実験結果を表示（大きな入力値）
    // print_experiment_result(50 * INPUT_MAX_X, &Algorithm::SR2);
    // print_experiment_result(50 * INPUT_MAX_X, &Algorithm::SR3);
}

// 実験結果を出力する関数
pub fn print_experiment_result(max_x: i128, select_algorithm: &Algorithm) {
    let algorithm_name: &str = get_algorithm_name(select_algorithm);
    println!("{} の実験結果", algorithm_name);
    println!("入力値 x の範囲: 1 ~ {}", max_x);

    let datas: Vec<(i128, u128)> = get_experiment_datas(max_x, select_algorithm);
    for (x, time) in datas {
        println!("{}, {}", x, time);
    }
}

// 実験データを返す関数
pub fn get_experiment_datas(max_x: i128, select_algorithm: &Algorithm) -> Vec<(i128, u128)> {
    let algorithm: fn(i128) -> i128 = get_algorithm_function(select_algorithm);

    let mut datas: Vec<(i128, u128)> = Vec::new();
    for x in 1i128..=max_x {
        let start_time: ProcessTime = ProcessTime::now(); // 計測開始
        let _res: i128 = algorithm(x);
        let duration: Duration = start_time.elapsed(); // 計測終了（計測結果）
        datas.push((x, duration.as_micros()));
    }

    datas
}