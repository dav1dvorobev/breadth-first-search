import pandas as pd
import matplotlib.pyplot as plt
import glob

path = 'output/data-*.csv'
files = glob.glob(path)
data_frames = []
for file in files:
    df = pd.read_csv(file)
    data_frames.append(df)
data = pd.concat(data_frames, ignore_index=True)

avg_data = data.groupby('v').mean()

v_values = avg_data.index
theoretical_v_plus_e_1 = v_values + v_values
theoretical_v_plus_e_2 = v_values + v_values * (v_values - 1) / 4
theoretical_v_plus_e_3 = v_values + v_values * (v_values - 1) / 2

fig, ax1 = plt.subplots(figsize=(10, 6))

ax1.plot(avg_data.index, avg_data['best'], label='Best Case', color='green')
ax1.plot(avg_data.index, avg_data['medium'], label='Medium Case', color='blue')
ax1.plot(avg_data.index, avg_data['worst'], label='Worst Case', color='red')

ax1.set_xlabel('V (Number of Vertices)')
ax1.set_ylabel('Execution Time (nanoseconds)')
ax1.tick_params(axis='y', labelcolor='black')
ax1.legend(loc='upper left')

ax2 = ax1.twinx()
ax2.plot(v_values, theoretical_v_plus_e_1, label='E = V', linestyle='--', color='darkgreen')
ax2.plot(v_values, theoretical_v_plus_e_2, label='E = V*(V-1)/4', linestyle='--', color='darkblue')
ax2.plot(v_values, theoretical_v_plus_e_3, label='E = V*(V-1)/2', linestyle='--', color='darkred')

ax2.set_ylabel('Theoretical O(V + E)', color='black')
ax2.tick_params(axis='y', labelcolor='black')
ax2.legend(loc='upper right')

plt.tight_layout()
plt.show()