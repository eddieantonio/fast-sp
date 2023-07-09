#include <assert.h>
#include <stddef.h>
#include <stdint.h>

// ssize_t is not a part of the C standard, so we have to define it:
typedef uint64_t ssize_t;
static_assert(sizeof(ssize_t) == sizeof(size_t), "Not the right size, whoops");

ssize_t count_c(const char *s) {
    ssize_t result = 0;
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
