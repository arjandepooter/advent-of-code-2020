import sys
from typing import List, Set, Tuple
from math import sqrt
import numpy as np

Grid = List[List[bool]]


class Tile:
    def __init__(self, iden: int, grid: Grid):
        self.iden = iden
        self.grid = grid

    def edges(self, rotation: int, flipped: bool) -> List[int]:
        grid = self.grid
        edges = [
            self.grid[0][::],
            [row[-1] for row in self.grid],
            self.grid[-1][::],
            [row[0] for row in self.grid],
        ]

        if flipped:
            edges[0], edges[2] = edges[2], edges[0]
            edges[1].reverse()
            edges[3].reverse()

        for _ in range(rotation):
            edges = edges[3:] + edges[:3]
            edges[0].reverse()
            edges[2].reverse()

        return [self._edge_to_int(edge) for edge in edges]

    def _edge_to_int(self, edge: List[bool]) -> int:
        n = 0
        for c in edge:
            n <<= 1
            n |= int(c)

        return n

    def __str__(self) -> str:
        return f"{self.iden}"

    def __repr__(self) -> str:
        return f"Tile({str(self)})"

    def __eq__(self, o: object) -> bool:
        if not isinstance(o, Tile):
            raise NotImplementedError()
        return o.iden == self.iden

    def __hash__(self) -> int:
        return hash(self.iden)


def backtrack(
    tiles: Set[Tile], current: List[Tuple[Tile, int, bool]]
) -> List[Tuple[Tile, int, bool]]:
    available_tiles = tiles - set(tile for (tile, _, _) in current)
    width = int(sqrt(len(tiles)))

    if len(available_tiles) == 0:
        return []

    (left, top) = None, None
    if len(current) % width > 0:
        prev = current[-1]
        left = prev[0].edges(prev[1], prev[2])[1]
    if len(current) // width > 0:
        prev = current[-width]
        top = prev[0].edges(prev[1], prev[2])[2]

    for new_tile in available_tiles:
        for rotation in range(4):
            for flipped in (True, False):
                edges = new_tile.edges(rotation, flipped)

                if (left and left != edges[3]) or (top and top != edges[0]):
                    continue

                nxt = current + [(new_tile, rotation, flipped)]
                sol = backtrack(tiles, nxt)
                if sol != None:
                    return [(new_tile, rotation, flipped)] + sol


def parse_input(data: str) -> Set[Tile]:
    result = set()
    for block in data.split("\n\n"):
        lines = block.splitlines()
        iden = int(lines[0].split(" ")[1].strip(":"))
        grid = [[c == "#" for c in line] for line in lines[1:]]
        result.add(Tile(iden, grid))

    return result


def solve_a(tiles: List[Tile]) -> int:
    width = int(sqrt(len(tiles)))
    solution = [tile.iden for (tile, _, _) in backtrack(tiles, [])]

    return solution[0] * solution[width - 1] * solution[-width] * solution[-1]


def solve_b(tiles: List[Tile]) -> int:
    width = int(sqrt(len(tiles)))
    fills = [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ]
    pattern = np.repeat(False, 60).reshape(3, 20)
    for p in fills:
        pattern[p] = True

    solution = backtrack(tiles, [])

    grids = []
    for tile, rot, flipped in backtrack(tiles, []):
        grid = np.array(tile.grid)

        if flipped:
            grid: np.ndarray = np.flipud(grid)

        grid = np.rot90(grid, rot, axes=(1, 0))
        grid = grid[1:-1, 1:-1]
        grids.append(grid)

    grid_width = len(grids[0])
    image = (
        np.array(grids)
        .reshape(width, width, grid_width, grid_width)
        .transpose(0, 2, 1, 3)
        .reshape(width * grid_width, -1)
    )

    width = width * grid_width
    found = set()
    for flipped in (False, True):
        for rotation in range(4):
            patt = pattern.copy()
            if flipped:
                patt = np.flipud(patt)
            patt = np.rot90(patt, rotation, axes=(1, 0))

            di, dj = patt.shape
            for i in range(width - di):
                for j in range(width - dj):
                    sub = image[i : i + di, j : j + dj]
                    if np.array_equal(patt, sub & patt):
                        for ddi, ddj in np.ndindex(patt.shape):
                            if patt[ddi][ddj]:
                                found.add((ddi + i, ddj + j))

    return np.count_nonzero(image) - len(found)


def test_edges():
    tile = Tile(
        123,
        [
            [True, True, False, True],
            [False, False, False, False],
            [True, False, False, False],
            [False, False, False, True],
        ],
    )
    assert tile.edges(0, False) == [13, 9, 1, 10]
    assert tile.edges(0, True) == [1, 9, 13, 5]
    assert tile.edges(1, False) == [5, 13, 9, 1]
    assert tile.edges(2, False) == [8, 5, 11, 9]
    assert tile.edges(4, True) == tile.edges(0, True)


if __name__ == "__main__":
    data = parse_input(sys.stdin.read())
    print(solve_a(data))
    print(solve_b(data))
