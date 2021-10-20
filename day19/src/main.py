import sys
from functools import reduce
from typing import Dict, List, Tuple, Union

Ruleset = Dict[int, Union[List[List[int]], str]]


def parse_input(input: str) -> Tuple[Ruleset, List[str]]:
    rules, lines = input.split("\n\n", 2)
    lines = lines.splitlines()

    ruleset = {}
    for rule in rules.splitlines():
        s, n = rule.split(": ")
        s = int(s)
        ruleset[s] = []
        for d in n.split(" | "):
            if d.startswith('"'):
                ruleset[s] = d.strip('"')
            else:
                ruleset[s] += [[int(c) for c in d.split(" ")]]

    return ruleset, lines


def check(ruleset: Ruleset, s: str, block: int = 0) -> List[str]:
    rules = ruleset[block]

    if isinstance(rules, str):
        return [s[1:]] if s.startswith(rules) else []

    solutions: List[str] = []
    for rule in rules:
        solutions += reduce(
            lambda prev_sols, b: [
                sol
                for sols in (check(ruleset, prev_sol, b) for prev_sol in prev_sols)
                for sol in sols
            ],
            rule,
            [s],
        )

    return solutions


def get_valid_solutions(ruleset: Ruleset, lines: List[str]):
    n = 0
    for line in lines:
        solutions = check(ruleset, line)
        if any(len(sol) == 0 for sol in solutions):
            n += 1
    return n


def solve_a(ruleset: Ruleset, lines: List[str]):
    return get_valid_solutions(ruleset, lines)


def solve_b(ruleset: Ruleset, lines: List[str]):
    ruleset[8] = [[42], [42, 8]]
    ruleset[11] = [[42, 31], [42, 11, 31]]

    return get_valid_solutions(ruleset, lines)


if __name__ == "__main__":
    ruleset, lines = parse_input(sys.stdin.read())
    print(solve_a(ruleset.copy(), lines))
    print(solve_b(ruleset.copy(), lines))
