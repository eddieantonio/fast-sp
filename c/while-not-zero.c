// Copyright (C) 2023  Eddie Antonio Santos
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#include <stddef.h>
#include <stdint.h>

/**
 * My implementation of Owen's C source code. I think most compilers (after
 * optimization passes) would produce more or less identical code for both
 * implementations.
 *
 * See assembly on Compiler Explorer: <https://godbolt.org/z/f7ve6K5sn>
 */
int64_t while_not_zero(const char *s) {
    int64_t result = 0;
    while (*s != '\0') {
        switch (*s) {
            case 's':
                result += 1;
                break;
            case 'p':
                result -= 1;
                break;
        }
        s++;
    }

    return result;
}
