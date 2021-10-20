from __future__ import annotations
import sys
from typing import Iterator, List, Set


class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def neighbours(self) -> Iterator[Point]:
        for x, y in [(-2, 0), (-1, -1), (1, -1), (2, 0), (1, 1), (-1, 1)]:
            yield self + Point(x, y)

    def __add__(self, o):
        if not isinstance(o, Point):
            raise NotImplementedError()

        return Point(self.x + o.x, self.y + o.y)

    def __eq__(self, o: object) -> bool:
        if not isinstance(o, Point):
            raise NotImplementedError()

        return self.x == o.x and self.y == o.y

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def __str__(self) -> str:
        return f"Point({self.x}, {self.y})"

    def __repr__(self) -> str:
        return str(self)


MAPPING = {
    "w": Point(-2, 0),
    "sw": Point(-1, -1),
    "se": Point(1, -1),
    "e": Point(2, 0),
    "ne": Point(1, 1),
    "nw": Point(-1, 1),
}

Path = List[Point]
Floor = Set[Point]


def parse_input(data: str) -> List[Path]:
    paths = []

    for line in data.splitlines():
        path = []

        while len(line) > 0:
            for d, o in MAPPING.items():
                if line.startswith(d):
                    path.append(o)
                    line = line[len(d) :]
                    break
        paths.append(path)

    return paths


def get_floor(paths: List[Path]) -> Floor:
    floor = set()

    for path in paths:
        p = Point(0, 0)

        for d in path:
            p += d

        floor ^= {p}

    return floor


def solve_a(paths: List[Path]) -> int:
    floor = get_floor(paths)

    return len(floor)


def step(floor: Floor) -> Floor:
    new_floor = set()

    to_consider = floor | set()
    for point in floor:
        to_consider |= set(point.neighbours())

    for p in to_consider:
        n = len([nb for nb in p.neighbours() if nb in floor])
        if (p not in floor and n == 2) or (p in floor and n in (1, 2)):
            new_floor.add(p)

    return new_floor


def solve_b(paths: List[Path]) -> int:
    floor = get_floor(paths)

    for i in range(100):
        floor = step(floor)
        print(f"Day {i + 1}: {len(floor)}")

    return len(floor)


if __name__ == "__main__":
    paths = parse_input(sys.stdin.read())
    print(solve_a(paths))
    print(solve_b(paths))
