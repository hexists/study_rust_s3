#!/usr/bin/env python3


a = 1
b = 1
print(a)
print(b)

for i in range(11):
    print(a + b)
    a, b = b, a + b
