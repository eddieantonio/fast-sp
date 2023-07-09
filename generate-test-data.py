# Copyright (C) 2023  Eddie Antonio Santos
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

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
