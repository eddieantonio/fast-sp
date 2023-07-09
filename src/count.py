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

import numpy as np  # type: ignore


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


def benchmark() -> None:
    import timeit
    from itertools import product
    from pathlib import Path
    from statistics import median

    # Load test data.
    # The test data will have been placed in target/release/build/<SOMETHING>/out
    # <SOMETHING> is determined by cargo, so it's easier just to try to glob for
    # something matching the expected filename.
    here = Path(__file__).parent.resolve()
    builds_dir = here.parent / "target" / "release" / "build"
    assert builds_dir.is_dir()
    (random_printable_path,) = builds_dir.glob("**/random-printable.bin")
    (random_sp_path,) = builds_dir.glob("**/random-sp.bin")

    random_printable = np.fromfile(random_printable_path, dtype=np.uint8)
    random_sp = np.fromfile(random_sp_path, dtype=np.uint8)

    def time(stmt: str, **kwargs):
        timer = timeit.Timer(stmt=stmt, **kwargs)
        k, _ = timer.autorange()
        raw_vector = timer.repeat(number=k)
        vector = [sec_per_k_iters * 10**9 / k for sec_per_k_iters in raw_vector]
        return k, vector, raw_vector

    # iter_naive, vector_naive = time("naive_for_loop(random_printable)")
    # iter_np, vector_np, raw_vector_np = time("clever_numpy_trickery(random_printable)")

    # fns = ["clever_numpy_trickery"]
    # test_cases = ["random_printable"]

    fns = ["naive_for_loop", "clever_numpy_trickery"]
    test_cases = ["random_printable", "random_sp"]

    for fn, test_case in product(fns, test_cases):
        namespace = globals() | {fn: globals()[fn], test_case: locals()[test_case]}
        k, vector, _ = time(f"{fn}({test_case})", globals=namespace)
        median_time = int(median(vector))
        min_time = min(vector)
        fake_stddev = int(median_time - min_time)
        print(f"{fn}::{test_case} ... bench: {median_time} ns/iter (+/- {fake_stddev})")


if __name__ == "__main__":
    benchmark()
