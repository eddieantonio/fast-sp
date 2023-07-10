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

import timeit
from dataclasses import dataclass
from functools import cached_property
from itertools import product
from pathlib import Path

import numpy as np  # type: ignore

HERE = Path(__file__).parent.resolve()


def naive_for_loop(array):
    result = 0
    for element in array:
        if element == b"s":
            result += 1
        elif element == b"p":
            result -= 1
    return result


def clever_numpy_trickery(array):
    num_s = np.count_nonzero(array == b"s")
    num_p = np.count_nonzero(array == b"p")
    return num_s - num_p


def just_finding_index(array):
    return array == b"s"


def load_test_data(name: str):
    # Load test data.
    # The test data will have been placed in target/release/build/<SOMETHING>/out
    # <SOMETHING> is determined by cargo, so it's easier just to try to glob for
    # something matching the expected filename.
    builds_dir = HERE.parent / "target" / "release" / "build"
    assert builds_dir.is_dir()
    (path,) = builds_dir.glob(f"**/{name}")
    return np.fromfile(path, dtype=np.uint8)


@dataclass
class Measurement:
    iterations: int
    samples_secs: np.ndarray

    @cached_property
    def mean_time(self) -> float:
        return np.mean(self.samples_secs)

    @property
    def min_time(self) -> float:
        return np.min(self.samples_secs)

    @property
    def fake_stddev(self) -> float:
        return self.mean_time - self.min_time


def as_nanos(seconds):
    return seconds * 10**9


def print_measurement(fn: str, test_case: str, measurement: Measurement):
    mean_time = int(as_nanos(measurement.mean_time))
    stddev = int(as_nanos(measurement.fake_stddev))
    print(f"{fn}::{test_case} ... bench: {mean_time} ns/iter (+/- {stddev})")


def time(stmt: str, **kwargs) -> Measurement:
    timer = timeit.Timer(stmt=stmt, **kwargs)
    k, _ = timer.autorange()
    raw_samples = timer.repeat(number=k)
    return Measurement(iterations=k, samples_secs=np.array(raw_samples) / k)


def benchmark() -> None:
    random_printable = load_test_data("random-printable.bin")
    random_sp = load_test_data("random-sp.bin")

    fns = [
        "naive_for_loop",
        "clever_numpy_trickery",
        "just_finding_index",
    ]
    test_cases = ["random_printable", "random_sp"]
    for fn, test_case in product(fns, test_cases):
        namespace = globals() | {fn: globals()[fn], test_case: locals()[test_case]}
        measurement = time(f"{fn}({test_case})", globals=namespace)
        print_measurement(fn, test_case, measurement)

    # Special case for np.count_nonzero
    for test_case in test_cases:
        namespace = globals() | {test_case: locals()[test_case]}
        measurement = time(
            f"np.count_nonzero(array)",
            setup=f"array = {test_case} == b's'",
            globals=namespace,
        )
        print_measurement("np.count_nonzero", test_case, measurement)


if __name__ == "__main__":
    benchmark()
