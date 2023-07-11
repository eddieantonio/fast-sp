#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

static_assert(sizeof(int) == 4, "Cannot assume that C's int is i32 in Rust");

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
