from collections import deque
import sys
from typing import Deque, Tuple


Deck = Deque[int]


def score(deck: Deck):
    s = 0
    for i, c in enumerate(reversed(deck), 1):
        s += i * c
    return s


def parse_input(data: str) -> Tuple[Deck, Deck]:
    p1, p2 = data.split("\n\n")

    return (
        deque(int(c) for c in p1.splitlines()[1:]),
        deque(int(c) for c in p2.splitlines()[1:]),
    )


def solve_a(deck1: Deck, deck2: Deck) -> int:
    while len(deck1) and len(deck2):
        c1 = deck1.popleft()
        c2 = deck2.popleft()

        winner = deck1 if c1 > c2 else deck2
        cards = [c1, c2] if c1 > c2 else [c2, c1]
        winner.extend(cards)

    return score(deck1) + score(deck2)


def play(deck1: Deck, deck2: Deck) -> Tuple[int, int]:
    rounds = set()

    while len(deck1) and len(deck2):
        key = (str(deck1), str(deck2))
        if key in rounds:
            return (0, 1)
        rounds.add(key)

        c1 = deck1.popleft()
        c2 = deck2.popleft()

        winner = 1 if c1 > c2 else 2

        if len(deck1) >= c1 and len(deck2) >= c2:
            n1 = deque(list(deck1)[:c1])
            n2 = deque(list(deck2)[:c2])
            _, winner = play(n1, n2)

        (deck1 if winner == 1 else deck2).extend([c1, c2] if winner == 1 else [c2, c1])

    s1 = score(deck1)
    s2 = score(deck2)

    return (s1 + s2, 1 if s1 > s2 else 2)


def solve_b(deck1: Deck, deck2: Deck) -> int:
    score, _ = play(deck1, deck2)

    return score


if __name__ == "__main__":
    p1, p2 = parse_input(sys.stdin.read())
    print(solve_a(p1.copy(), p2.copy()))
    print(solve_b(p1.copy(), p2.copy()))
