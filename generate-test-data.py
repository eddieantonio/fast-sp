"""
generate-test-data.py - creates random test data, quickly

SYNOPSIS
    python3 generate-test-data.py [-C directory]

OPTIONS
    -C directory      if specifed, changes into this directory before executing
"""

import os
import sys

import numpy as np  # type: ignore

KILOBYTE = 1024
MEGABYTE = 1024 * KILOBYTE
SIZE = 12 * MEGABYTE

generator = np.random.default_rng()


def random_sp():
    "Generate an array of random ASCII 's' or 'p' characters"
    s_or_p = np.array([ord("p"), ord("s")], dtype=np.uint8)
    return generator.choice(s_or_p, size=(SIZE,), shuffle=False)


def random_ascii_printable():
    "Generate an array of random printable ASCII characters"
    min_char = ord(" ")  # Lowest, printable ASCII char
    max_char = ord("~")  # Highest, printable ASCII char
    return generator.integers(
        low=min_char, high=max_char, endpoint=True, dtype=np.uint8, size=(SIZE,)
    )


if __name__ == "__main__":
    # Change directory, if specified:
    if flag_index := sys.argv.index("-C"):
        directory = sys.argv[flag_index + 1]
        os.chdir(directory)

    with open("random-printable.bin", "wb") as data_file:
        random_ascii_printable().tofile(data_file)

    with open("random-sp.bin", "wb") as data_file:
        random_sp().tofile(data_file)
