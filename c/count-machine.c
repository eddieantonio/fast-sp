#include <stdio.h>
#include <stdint.h>

enum {
    COUNT_OTHER = 0,
    COUNT_S = 1,
    COUNT_P = 2,
};

static uint8_t categorize[256] = {
    ['s'] = COUNT_S,
    ['p'] = COUNT_P,
    COUNT_OTHER
};

int c_count_machine(char *input, size_t n) {
    unsigned int counts[3] = { 0 };

    for (size_t i = 0; i < n; i++) {
        uint8_t c = input[i];
        counts[categorize[c]]++;
    }

    int result = counts[COUNT_S] - counts[COUNT_P];
    return result;
}
