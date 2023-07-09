#include <stdbool.h>
#include <assert.h>
#include <stdint.h>

/**
 * My implementation of Owen's C source code. I think most compilers (after
 * optimization passes) would produce more or less identical code for both
 * implementations.
 */
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

static_assert(sizeof(int) == 4, "Cannot assume that C's int is i32 in Rust");
/**
 * Owen Sheppard's original implementation.
 * From: https://owen.cafe/posts/six-times-faster-than-c/
 */
int run_switches(char *input) {
  int res = 0;
  while (true) {
    char c = *input++;
    switch (c) {
      case '\0':
        return res;
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
}
