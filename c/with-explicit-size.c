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
 * See assembly in Compiler Explorer: <https://godbolt.org/z/aPGWTW8WP>
 */
int with_explicit_size(const char *input, size_t n) {
    int res = 0;
    for (size_t i = 0; i < n; i++) {
        char c = input[i];
        switch (c) {
            case 's':
                res += 1;
                break;
            case 'p':
                res -= 1;
                break;
            default:
                break;
        }
    }
    return res;
}
