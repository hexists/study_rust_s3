#!/usr/bin/env python3


for i in range(1, 10):
    for j in range(1, 10):
        print(f"{i * j:3d}", end='')
        if j < 9:
            print(", ", end='')
    print()
