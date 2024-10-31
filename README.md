# algo_comp_exp_code1
アルゴリズムと計算量特論の実験レポートで用いるプログラムのコード.

Rust 言語のプログラムを実行するためには, 事前に `cargo` を用意する必要がある.
これ以降では, Python 言語および Rust 言語の環境構築が済んでいることを前提に述べる.

# ディレクトリ構成
ディレクトリ構成の中から重要なもののみを抜粋すると以下の通りである.
<pre>
.
├── src
│   ├── lib.rs
│   └── main.rs
├── tests
│   └── lib_test.rs
├── graph.py
├── output1.txt
├── output2.txt
├── output2_big.txt
├── output3.txt
└── output3_big.txt
</pre>

- `./src/lib.rs` : Rust 言語で $3$ つのアルゴリズム SR1, SR2, SR3 を関数として実装したもの.
- `./src/main.rs` : アルゴリズムの処理時間の計測（実験）を行うコード. 実験結果は `output1.txt` 等である（ `cargo run > output1.txt` で実行）.
- `./tests/lib_test.rs` : `lib.rs` で実装した関数が正しいかのテストを行うテストコード（ `cargo test` でテスト実行）.
- `./graph.py` : Rust 言語で実装した各アルゴリズムの処理時間をグラフ化するための Python 言語のコード（`python graph.py` で実行）.
- `./output1.txt` : SR1 の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 1000)$
- `./output2.txt` : SR2 の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 1000)$
- `./output2_big.txt` : SR2 の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 50000)$
- `./output3.txt` : SR3 の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 1000)$
- `./output3_big.txt` : SR3 の処理時間 $($ 入力値 $x$ の範囲 : $1 \le x \le 50000)$
