import math

def f(x,n):
    val = str(x)
    if not '.' in val:
        val += '.0'
    print(f'F({val},"{n}"), ',end='')

f(256,"4m16s")
f(64, "1m6s")
f(16, "16s")
f(4, "4s")
f(1, "1s")
f(1/4, "1/4")
print()

def n(x):
    f(440 * 2**(x-4 - 9/12), f'C{x}')
    f(440 * 2**(x-4 - 8/12), f'C#{x}')
    f(440 * 2**(x-4 - 7/12), f'D{x}')
    f(440 * 2**(x-4 - 6/12), f'D#{x}')
    f(440 * 2**(x-4 - 5/12), f'E{x}')
    f(440 * 2**(x-4 - 4/12), f'F{x}')
    f(440 * 2**(x-4 - 3/12), f'F#{x}')
    f(440 * 2**(x-4 - 2/12), f'G{x}')
    f(440 * 2**(x-4 - 1/12), f'G#{x}')
    f(440 * 2**(x-4       ), f'A{x}')
    f(440 * 2**(x-4 + 1/12), f'A#{x}')
    f(440 * 2**(x-4 + 2/12), f'B{x}')
    print()

n(0)
n(1)
n(2)
n(3)
n(4)
n(5)
n(6)
n(7)
n(8)
