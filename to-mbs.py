#!/usr/bin/env python3

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
to-mbs.py - print ns/iter in a terms of bytes/sec.

SYNOPSIS
    python3 to-mbs.py [NUM ...]
"""

import ast
import sys

data_size_mb = 12  # size in MiB of data
numerator = 10**9 * data_size_mb


def transform(strings):
    for string in strings:
        as_literal = string.replace(",", "_")
        nanos_per_iter = ast.literal_eval(as_literal)
        result = numerator / nanos_per_iter  # in MiB/s
        if result < 1500.0:
            yield f"{result:.2f} MiB/s"
        else:
            result_gbs = result / 1024
            yield f"{result_gbs:.2f} GiB/s"


if len(sys.argv) > 1:
    print(*transform(sys.argv[1:]), sep="\t")
