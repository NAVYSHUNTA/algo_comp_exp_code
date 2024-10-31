import matplotlib.pyplot as plt
import numpy as np

# 次のコマンドで日本語フォントをインストール（グラフにある日本語が文字化けしないように表示）
# pip install japanize-matplotlib
import japanize_matplotlib


# フォントサイズを設定
plt.rcParams['font.size'] = 15

# 出力ファイル名とアルゴリズム名、線の色の対応
get_algorithm_name_and_line_color = {
    'output1.txt': ['SR1', 'red'],
    'output2.txt': ['SR2', 'green'],
    'output3.txt': ['SR3', 'blue'],
    'output2_big.txt': ['SR2', 'green'],
    'output3_big.txt': ['SR3', 'blue'],
}

# SR1, SR2, SR3 のうち指定されたものを比較するための図を作成する関数
def get_graph(*file_names):
    for file_name in file_names:
        data = np.loadtxt(file_name, skiprows=2, delimiter=',')
        algorithm_name, line_color = get_algorithm_name_and_line_color[file_name]
        plt.plot(data[:,0], data[:,1], label=algorithm_name, color=line_color)

    plt.xlabel('x の値') # x 軸のラベル
    plt.ylabel('処理時間 [μs]') # y 軸のラベル
    plt.legend() # 凡例を表示
    plt.tight_layout()
    plt.show()

get_graph('output1.txt', 'output2.txt', 'output3.txt')
get_graph('output2_big.txt', 'output3_big.txt')