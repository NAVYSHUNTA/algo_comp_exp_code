use algo_comp_exp_code1::*;

// アルゴリズム SR1 の単体テスト
#[test]
fn test_sr1_succeeds() {
    assert_eq!(1i128, sr1(1i128));
    assert_eq!(6i128, sr1(2i128));
    assert_eq!(18i128, sr1(3i128));

    assert_ne!(39i128, sr1(4i128));
    assert_ne!(76i128, sr1(5i128));
    assert_ne!(127i128, sr1(6i128));
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
    assert_eq!(1i128, sr2(1i128));
    assert_eq!(6i128, sr2(2i128));
    assert_eq!(18i128, sr2(3i128));

    assert_ne!(39i128, sr2(4i128));
    assert_ne!(76i128, sr2(5i128));
    assert_ne!(127i128, sr2(6i128));
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
    assert_eq!(1i128, sr3(1i128));
    assert_eq!(6i128, sr3(2i128));
    assert_eq!(18i128, sr3(3i128));

    assert_ne!(39i128, sr3(4i128));
    assert_ne!(76i128, sr3(5i128));
    assert_ne!(127i128, sr3(6i128));
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