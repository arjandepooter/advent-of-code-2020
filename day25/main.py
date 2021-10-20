from itertools import count


def encrypt(key: int, subject: int):
    return pow(subject, key, 20201227)


def find_key(pub: int) -> int:
    for n in count(1):
        if encrypt(n, 7) == pub:
            return n


def solve_a() -> int:
    pub1 = 5290733
    pub2 = 15231938

    key1 = find_key(pub1)
    key2 = find_key(pub2)
    print(key1, key2)
    assert encrypt(key1, pub2) == encrypt(key2, pub1)

    return encrypt(key1, pub2)


if __name__ == "__main__":
    print(solve_a())
