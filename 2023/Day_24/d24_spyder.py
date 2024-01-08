# -*- coding: utf-8 -*-
"""
Created on Mon Jan  8 00:43:26 2024

@author: Ponjinge
"""

import re
s = []

with open('./test.txt', 'r') as file:
    file_content = file.read().strip()  
individual_strings = file_content.split('\n')

for i, individual_str in enumerate(individual_strings, 1):
    word = [int(num) for num in re.split(", | @ ", individual_str)]
    s.append(word)

s.append([24,13,10,-3,1,2])
    
for i in range (len(s)):
    for k in range(3, len(s[i])):
        s[i][k] = s[i][k] * 10


import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# Your data
data = s

# Extract starting positions and movements
starting_positions = [row[:3] for row in data]
movement_vectors = [row[3:] for row in data]

# Calculate ending positions
ending_positions = []
for start, move in zip(starting_positions, movement_vectors):
    end = [sum(pair) for pair in zip(start, move)]
    ending_positions.append(end)

# Create a 3D scatter plot
fig = plt.figure(figsize=(8, 6))
ax = fig.add_subplot(111, projection='3d')

# Plot starting positions
ax.scatter(*zip(*starting_positions), color='blue', label='Starting Position', marker='o')

# Plot movement vectors
for i in range(len(starting_positions)):
    ax.quiver(*starting_positions[i], *movement_vectors[i], color='red', length=1, arrow_length_ratio=0.1)

# Set labels and title
ax.set_xlabel('X-axis')
ax.set_ylabel('Y-axis')
ax.set_zlabel('Z-axis')
ax.set_title('Vector Movements in 3D Space')

# Show legend
ax.legend()

# Show the plot
plt.show()
