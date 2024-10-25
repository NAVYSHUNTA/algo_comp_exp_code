import matplotlib.pyplot as plt
import numpy as np

# 次のコマンドで日本語フォントをインストール（グラフ内にある日本語を文字化けしないように表示）
# pip install japanize-matplotlib
import japanize_matplotlib


# フォントサイズを設定
plt.rcParams['font.size'] = 15

# SR1, SR2, SR3 の比較
data1 = np.loadtxt('output1.txt', skiprows=2, delimiter=',')
data2 = np.loadtxt('output2.txt', skiprows=2, delimiter=',')
data3 = np.loadtxt('output3.txt', skiprows=2, delimiter=',')

plt.plot(data1[:,0], data1[:,1], label='SR1', color='red')
plt.plot(data2[:,0], data2[:,1], label='SR2', color='green')
plt.plot(data3[:,0], data3[:,1], label='SR3', color='blue')

plt.xlabel('x の値') # x 軸のラベル
plt.ylabel('処理時間 [μs]') # y 軸のラベル
plt.legend() # 凡例を表示
plt.tight_layout()
plt.show()

# SR2, SR3 の比較
bigdata2 = np.loadtxt('output2_big.txt', skiprows=2, delimiter=',')
bigdata3 = np.loadtxt('output3_big.txt', skiprows=2, delimiter=',')

plt.plot(bigdata2[:,0], bigdata2[:,1], label='SR2', color='green')
plt.plot(bigdata3[:,0], bigdata3[:,1], label='SR3', color='blue')

plt.xlabel('x の値') # x 軸のラベル
plt.ylabel('処理時間 [μs]') # y 軸のラベル
plt.legend() # 凡例を表示
plt.tight_layout()
plt.show()