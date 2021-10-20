import pytest
import sys
from typing import Deque, List
from collections import deque

Data = Deque[int]


def parse_input(data: str) -> Data:
    return deque(int(c) for c in data.strip())


def to_int(l: Data) -> int:
    l = list(l)
    idx = l.index(1)
    result = 0
    for i, c in enumerate(reversed(l[idx + 1 :] + l[:idx])):
        result += c * 10 ** i
    return result


def move(data: Data, n=1) -> Data:
    l = len(data)

    for i in range(n):
        p = data.popleft()
        grab = [data.popleft(), data.popleft(), data.popleft()]
        c = (p - 1) % l
        while c in grab or c == 0:
            c = (c - 1) % (l + 1)
        idx = data.index(c)
        data.rotate(-idx - 1)
        data.extendleft(reversed(grab))
        data.rotate(idx + 1)
        data.append(p)

    return data


def solve_a(data: Data) -> int:
    data = data.copy()
    data = move(data, n=100)

    return to_int(data)


def solve_b(data: Data) -> int:
    data = data.copy()
    data.extend(range(len(data) + 1, 1000001))
    # data = move(data, n=10000000)
    data = move(data, n=10000)

    n = data.index(1)
    return data[n + 1 % len(data)] * data[n + 2 % len(data)]


if __name__ == "__main__":
    data = parse_input(sys.stdin.read())
    print(solve_a(data))
    print(solve_b(data))


def test_to_int():
    n = deque([5, 8, 3, 7, 4, 1, 9, 2, 6])
    assert to_int(n) == 92658374


@pytest.mark.parametrize(
    "moves,expected",
    [
        (1, deque([2, 8, 9, 1, 5, 4, 6, 7, 3])),
        (2, deque([5, 4, 6, 7, 8, 9, 1, 3, 2])),
        (3, deque([8, 9, 1, 3, 4, 6, 7, 2, 5])),
        (4, deque([4, 6, 7, 9, 1, 3, 2, 5, 8])),
        (5, deque([1, 3, 6, 7, 9, 2, 5, 8, 4])),
        (6, deque([9, 3, 6, 7, 2, 5, 8, 4, 1])),
        (7, deque([2, 5, 8, 3, 6, 7, 4, 1, 9])),
        (8, deque([6, 7, 4, 1, 5, 8, 3, 9, 2])),
        (9, deque([5, 7, 4, 1, 8, 3, 9, 2, 6])),
        (10, deque([8, 3, 7, 4, 1, 9, 2, 6, 5])),
    ],
)
def test_move(moves, expected):
    n = deque([3, 8, 9, 1, 2, 5, 4, 6, 7])
    assert move(n.copy(), moves) == expected
