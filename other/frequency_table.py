import math

def f(x,n):
    val = str(x)
    if not '.' in val:
        val += '.0'
    print(f'("{n}",{val}), ',end='')

def g(x):
    y = 1/x
    if x<1:
        if (y * 3) % 3 > 0:
            return f(y, "3/" + str(int(3*y)))
        return f(y, "1/" + str(int(y)))
    if x>60:
        return f(y, f'{x // 60}m{x % 60}s')
    return f(y, f'{x % 60}s')


g(256)
g(192)
g(128)
g(96)
g(64)
g(48)
g(32)
g(24)
g(16)
g(12)
g(8)
g(6)
g(4)
g(3)
g(2)
print()
g(3/2)
g(1)
g(3/4)
g(1/2)
g(3/8)
g(1/4)
g(3/16)
g(1/8)
g(3/32)
g(1/16)
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
