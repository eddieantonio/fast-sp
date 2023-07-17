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

#include <assert.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

int c_emulate_numpy(const char *buffer, size_t n) {
    int_fast8_t * s = (int_fast8_t *) malloc(n * sizeof(int_fast8_t));
    assert(s != NULL);
    int_fast8_t * p = (int_fast8_t *) malloc(n * sizeof(int_fast8_t));
    assert(p != NULL);

    for (size_t i = 0; i < n; ++i) {
        s[i] = buffer[i] == 's';
#if !FUSE_EQ_LOOP
    }
    for (size_t i = 0; i < n; ++i) {
#endif
        p[i] = buffer[i] == 'p';
    }

    int n_s = 0;
    for (size_t i = 0; i < n; ++i) {
        if (s[i] != 0) n_s += 1;
    }
    int n_p = 0;
    for (size_t i = 0; i < n; ++i) {
        if (p[i] != 0) n_p += 1;
    }

    free(s);
    free(p);

    return n_s - n_p;
}
