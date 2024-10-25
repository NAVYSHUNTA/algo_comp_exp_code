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