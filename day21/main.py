from functools import reduce
import sys
from typing import List, Tuple

Data = List[Tuple[List[str], List[str]]]


def parse_input(data: str) -> Data:
    result = []

    for line in data.splitlines():
        p1, p2 = line.split(" (contains ")
        ingredients = p1.split(" ")
        allergens = p2.strip(")").split(", ")
        result.append((ingredients, allergens))

    return result


def solve_a(data: Data):
    all_ingredients = set(
        [ingredient for (ingredients, _) in data for ingredient in ingredients]
    )
    all_allergens = set([allergen for (_, allergens) in data for allergen in allergens])

    a = {allergen: all_ingredients.copy() for allergen in all_allergens}

    for ingredients, allergens in data:
        for allergen in allergens:
            a[allergen] = a[allergen].intersection(set(ingredients))

    ags = all_ingredients - reduce(lambda a, b: a | b, a.values(), set())

    n = 0
    for ig in ags:
        for (igs, _) in data:
            for ig2 in igs:
                if ig2 == ig:
                    n += 1
    print(n)

    changed = True
    while changed:
        changed = False
        for ingredients in a.values():
            if len(ingredients) == 1:
                ingredient = list(ingredients)[0]
                for i2 in a.values():
                    if len(i2) != 1:
                        if ingredient in i2:
                            i2.remove(ingredient)
                            changed = True

    lst = sorted([(a, list(b)[0]) for a, b in a.items()])
    print(",".join(b for _, b in lst))


if __name__ == "__main__":
    data = parse_input(sys.stdin.read())
    solve_a(data)
