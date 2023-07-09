#include <stdint.h>

int64_t count_c(const char *s) {
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
