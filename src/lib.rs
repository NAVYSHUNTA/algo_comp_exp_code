// SR1(x)
// 入力: 正の整数 x
// 出力: 1 から x までの自然数の和を x 倍した値
// 計算量: O(x^2)
pub fn sr1(x: i128) -> i128 {
    if x <= 0i128 {
        panic!("正の整数を入力してください.");
    }

    let mut z: i128 = 0;
    for _i in 1i128..=x {
        for j in 1i128..=x {
            z = z + j;
        }
    }

    z
}

// SR2(x)
// 入力: 正の整数 x
// 出力: 1 から x までの自然数の和を x 倍した値
// 計算量: O(x)
pub fn sr2(x: i128) -> i128 {
    if x <= 0i128 {
        panic!("正の整数を入力してください.");
    }

    let mut z: i128 = 0;
    for i in 1i128..=x {
        z = z + i;
    }
    z = z * x;

    z
}

// SR3(x)
// 入力: 正の整数 x
// 出力: 1 から x までの自然数の和を x 倍した値
// 計算量: O(1)
pub fn sr3(x: i128) -> i128 {
    if x <= 0i128 {
        panic!("正の整数を入力してください.");
    }

    let z: i128 = x * x * (x + 1i128) / 2i128;

    z
}

// 列挙型（例えば SR1 で実験する際は &Algorithm::SR1 で指定する）
// なぜこうしたか？ : 関数（アルゴリズム）と関数名（アルゴリズム名）を渡す代わりに列挙型を渡すだけで済むようにしている（ヒューマンエラーの防止）.
pub enum Algorithm {
    SR1,
    SR2,
    SR3,
}

// アルゴリズムのアルゴリズム名を返す関数
pub fn get_algorithm_name(select_algorithm: &Algorithm) -> &str {
    match &select_algorithm {
        Algorithm::SR1 => "SR1",
        Algorithm::SR2 => "SR2",
        Algorithm::SR3 => "SR3",
    }
}

// アルゴリズム（関数）を返す関数
pub fn get_algorithm_function(select_algorithm: &Algorithm) -> fn(i128) -> i128 {
    match select_algorithm {
        Algorithm::SR1 => sr1,
        Algorithm::SR2 => sr2,
        Algorithm::SR3 => sr3,
    }
}