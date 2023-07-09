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
find-libcount.py - print the path to libcount.a (release mode)
"""

import sys
from pathlib import Path

here = Path(__file__).parent.resolve()
builds_dir = (
    here / "target" / "release" / "build"
)  # will be in / "fast-sp-063b01e3731a8eac" / "out"
assert builds_dir.is_dir()
options = list(builds_dir.glob("**/libcount.a"))

if len(options) == 0:
    print("libcount.a not found", file=sys.stderr)
    sys.exit(1)

if len(options) > 1:
    print(f"too many options for libcount.a: {options}", file=sys.stderr)
    sys.exit(1)

# Print the only option!
print(options[0])
