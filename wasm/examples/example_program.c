#include <string.h>
#include "intrinsics.h"

void poll() {
    const char *message = "Hello, World!";

    wasm_log(LOG_INFO, __FILE__, strlen(__FILE__), __LINE__, message, strlen(message));
}