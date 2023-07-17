#include <stdio.h>
#include <stdint.h>

enum {
    COUNT_OTHER = 0,
    COUNT_S = 1,
    COUNT_P = 2,
};

static uint8_t states[3][3] = {
    [COUNT_OTHER] = { COUNT_OTHER, COUNT_S, COUNT_P},
    [COUNT_S] = { COUNT_OTHER, COUNT_S, COUNT_P},
    [COUNT_P] = { COUNT_OTHER, COUNT_S, COUNT_P},
};

static uint8_t col[256] = {
    ['s'] = 1,
    ['p'] = 2,
    0
};

int c_state_machine(char *input, size_t n) {
    unsigned int counts[3] = { 0 };

    int state = COUNT_OTHER;
    for (size_t i = 0; i < n; i++) {
        uint8_t c = input[i];
        state = states[state][col[c]];
        counts[state]++;
    }

    int result = counts[COUNT_S] - counts[COUNT_P];
    return result;
}
