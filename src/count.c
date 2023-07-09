#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

static_assert(sizeof(int) == 4, "Cannot assume that C's int is i32 in Rust");

/**
 * My implementation of Owen's C source code. I think most compilers (after
 * optimization passes) would produce more or less identical code for both
 * implementations.
 *
 * See assembly on Compiler Explorer: <https://godbolt.org/z/4cEqo1vbe>
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

/**
 * Owen Sheppard's original implementation.
 * From: https://owen.cafe/posts/six-times-faster-than-c/
 *
 * See assembly on Compiler Explorer: https://godbolt.org/z/4cqoqdbTq
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

/**
 * See assembly in Compiler Explorer: <https://godbolt.org/z/W3jchbsab>
 */
int count_c_owen_sized(const char *input, size_t n) {
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
