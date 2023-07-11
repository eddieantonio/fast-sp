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
        *_, implementation, benchmark = path.split("::")

        # There is a stray ")" in the output
        stddev = stddev.rstrip(")")
        if benchmark.startswith("bench_"):
            test_case = benchmark.removeprefix("bench_")
        else:
            test_case = benchmark

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


def add_gigabytes_per_second(df):
    return df.assign(gigabytes_per_second=df["bytes_per_second"] / 1024**3)


def add_language_and_name(df):
    def extract(implementation_name):
        first_part, _, name = implementation_name.partition("_")

        if first_part in ("c", "rust", "python"):
            return first_part.capitalize(), name
        elif first_part in ("vec", "nonzero"):
            # I forgot to add the language name for these
            return "Rust", implementation_name
        elif first_part.startswith("np"):
            return "Python", implementation_name
        raise ValueError(implementation_name)

    result = df["implementation"].apply(extract).apply(pd.Series)
    languages, names = result[0], result[1]
    return df.assign(language=languages, name=names)


def add_category(df):
    """
    Naming things is hard. "Category" is whether the bencmarks tests a full solution or
    just testing a part of an implementation.
    """

    def categorize(name):
        if "nonzero" == name or "vec_eq" in name or name.startswith("np."):
            return "Part"
        return "Full"

    return df.assign(category=df["implementation"].apply(categorize))


def nullify_test_case_size_conditionally(df):
    weird_cases = ("vec_eq_do_nothing_but_allocate", "vec_eq_only_prefix")
    is_weird_case = df["implementation"].apply(lambda name: name not in weird_cases)
    return df.assign(
        bytes_per_iteration=df["bytes_per_iteration"].where(is_weird_case, pd.NA)
    )


######################################### Main #########################################

rows = list(parse(fileinput.input(encoding="UTF-8")))

benchmarks = (
    pd.DataFrame(rows, columns=["implementation", "test_case", "mean_ns", "stddev_ns"])
    .pipe(add_language_and_name)
    .pipe(add_category)
    .pipe(add_test_case_size)
    .pipe(nullify_test_case_size_conditionally)
    .pipe(add_bytes_per_second)
    .pipe(add_gigabytes_per_second)
)


def with_escaped_markdown(df):
    def escape_markdown(name):
        return name.replace("_", r"\_")

    return df.assign(
        name=df["name"].apply(escape_markdown),
        test_case=df["test_case"].apply(escape_markdown),
    )


def with_preformatted_time(df):
    formatted_mean = df["mean_ns"].apply(lambda x: f"{x:,}")
    formatted_stddev = df["stddev_ns"].apply(lambda x: f"{x:,}")

    mean_width = formatted_mean.apply(len).max()
    stddev_width = formatted_stddev.apply(len).max()

    def format_row():
        for mean, stddev in zip(formatted_mean, formatted_stddev):
            yield f"{mean:>{mean_width}} ns/iter ± {stddev:>{stddev_width}}"

    return df.assign(time=pd.Series(format_row()))


presentable = (
    benchmarks.query("category == 'Full'")
    .sort_values("mean_ns")
    .reset_index()
    .pipe(with_escaped_markdown)
    .pipe(with_preformatted_time)
)


print(
    presentable[
        [
            "language",
            "name",
            "test_case",
            "gigabytes_per_second",
            "time",
        ]
    ].to_markdown(
        headers=[
            "Language",
            "Implementation",
            "Test case",
            "Throughput (GiB/s)",
            "Time per iteration",
        ],
        colalign=["left", "left", "left", "right", "right"],
        index=False,
        floatfmt=".3f",
        intfmt=",",
    )
)
