"""
Generate test data.
"""

import numpy as np  # type: ignore

KILOBYTE = 1024
MEGABYTE = 1024 * KILOBYTE
SIZE = 12 * MEGABYTE

generator = np.random.default_rng()


def all_p():
    return np.full((SIZE,), ord("p"), dtype=np.uint8)


def all_s():
    return np.full((SIZE,), ord("s"), dtype=np.uint8)


def random_sp():
    s_or_p = np.array([ord("p"), ord("s")], dtype=np.uint8)
    return generator.choice(s_or_p, size=(SIZE,), shuffle=False)


def random_ascii_printable():
    min_char = ord(" ")  # Lowest, printable ASCII char
    max_char = ord("~")  # Highest, printable ASCII char
    return generator.integers(
        low=min_char, high=max_char, endpoint=True, dtype=np.uint8, size=(SIZE,)
    )


with open("random-printable.bin", "wb") as data_file:
   random_ascii_printable().tofile(data_file)

with open("random-sp.bin", "wb") as data_file:
   random_sp().tofile(data_file)
