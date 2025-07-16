import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Read JSON data
df = pd.read_json('rtt_stats.json', lines=True)

# Prepare data (x as timestamp, y as average RTT)
x = df['ts'].values
y = df['avg_rtt'].values

# Least squares fitting (linear equation: y = kx + b)
k, b = np.polyfit(x, y, 1)
y_fit = k * x + b

# Create plot
plt.figure(figsize=(10, 6))

# Plot original avg_rtt data (points connected by solid line)
plt.plot(x, y, marker='o', linestyle='-', color='blue', label='avg_rtt data')

# Plot least squares fitting line
plt.plot(x, y_fit, color='red', linestyle='--', label=f'Fitted line: y = {k:.10f}x + {b:.4f}')

# Set plot properties
plt.xlabel('Timestamp (ts)')
plt.ylabel('Average Round-Trip Time (avg_rtt)')
plt.title('avg_rtt Data with Least Squares Fitting Line')
plt.legend()
plt.grid(linestyle='--', alpha=0.7)

# Save and display plot
plt.savefig('avg_rtt_line_fit.png', dpi=100, bbox_inches='tight')
print("Plot saved as avg_rtt_line_fit.png")
plt.show()


