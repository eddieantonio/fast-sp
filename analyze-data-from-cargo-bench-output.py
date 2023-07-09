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

import fileinput
import functools
import sys
from pathlib import Path

import pandas as pd  # type: ignore

HERE = Path(__file__).parent.resolve()


def parse(lines):
    "Parse the lines of the `cargo bench` output"
    for line in lines:
        if "... bench:" not in line:
            continue

        _test, path, _elipsis, _bench, mean, _nsiter, _pm, stddev = line.split()
        _benches, implementation, benchmark = path.split("::")

        # There is a stray ")" in the output
        stddev = stddev.rstrip(")")
        test_case = benchmark[len("bench_") :]

        yield implementation, test_case, to_int(mean), to_int(stddev)


def to_int(measurement: str) -> int:
    return int(measurement.replace(",", ""))


def add_test_case_size(df):
    return df.assign(bytes_per_iteration=df["test_case"].apply(test_case_size))


@functools.cache
def test_case_size(test_case_name):
    """
    Return the size of the test case, in bytes.
    """
    # Find the test case. It will be somewhere in  ./target/releasee
    filename = test_case_name.replace("_", "-") + ".bin"
    builds_dir = HERE / "target" / "release" / "build"
    assert builds_dir.is_dir()
    matches = list(builds_dir.glob(f"**/{filename}"))
    assert (
        len(matches) == 1
    ), f"could not find exactly one test case called '{filename}'; found: {matches}"
    (test_case_path,) = matches

    # Get its size:
    return test_case_path.stat().st_size


def add_bytes_per_second(df):
    nanos_per_second = 10**9
    return df.assign(
        bytes_per_second=(1.0 / df["mean_ns"])
        * df["bytes_per_iteration"]
        * nanos_per_second
    )


######################################### Main #########################################

rows = list(parse(fileinput.input(encoding="UTF-8")))

benchmarks = (
    pd.DataFrame(rows, columns=["implementation", "test_case", "mean_ns", "stddev_ns"])
    .pipe(add_test_case_size)
    .pipe(add_bytes_per_second)
    .pipe(lambda df: df.assign(gigabytes_per_second=df["bytes_per_second"] / 1024**3))
)

print(
    benchmarks[
        [
            "implementation",
            "test_case",
            "gigabytes_per_second",
            "mean_ns",
            "stddev_ns",
        ]
    ].to_markdown(index=False, floatfmt=".3f")
)
