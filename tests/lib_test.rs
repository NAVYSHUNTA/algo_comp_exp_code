use algo_comp_exp_code::*;

// アルゴリズム SR1 の単体テスト
#[test]
fn test_sr1_succeeds() {
    assert_eq!(sr1(1i128), 1i128);
    assert_eq!(sr1(2i128), 6i128);
    assert_eq!(sr1(3i128), 18i128);

    assert_ne!(sr1(4i128), 39i128);
    assert_ne!(sr1(5i128), 76128);
    assert_ne!(sr1(6i128), 127i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr1_input_zero_panic() {
    sr1(0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr1_input_negative_one_panic() {
    sr1(-1i128);
}

// アルゴリズム SR2 の単体テスト
#[test]
fn test_sr2_succeeds() {
    assert_eq!(sr2(1i128), 1i128);
    assert_eq!(sr2(2i128), 6i128);
    assert_eq!(sr2(3i128), 18i128);

    assert_ne!(sr2(4i128), 39i128);
    assert_ne!(sr2(5i128), 76i128);
    assert_ne!(sr2(6i128), 127i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr2_input_zero_panic() {
    sr2(0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr2_input_negative_one_panic() {
    sr2(-1i128);
}

// アルゴリズム SR3 の単体テスト
#[test]
fn test_sr3_succeeds() {
    assert_eq!(sr3(1i128), 1i128);
    assert_eq!(sr3(2i128), 6i128);
    assert_eq!(sr3(3i128), 18i128);

    assert_ne!(sr3(4i128), 39i128);
    assert_ne!(sr3(5i128), 76i128);
    assert_ne!(sr3(6i128), 127i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr3_input_zero_panic() {
    sr3(0i128);
}

#[test]
#[should_panic(expected = "正の整数を入力してください.")]
fn test_sr3_input_negative_one_panic() {
    sr3(-1i128);
}