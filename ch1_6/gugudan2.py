#!/usr/bin/env python3


for i in range(1, 10):
    dan = [f"{i * j:3d}" for j in range(1, 10)]
    print(', '.join(dan))
