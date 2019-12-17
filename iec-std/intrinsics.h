#ifndef WASM_INTRINSICS_H
#define WASM_INTRINSICS_H

#include <stdbool.h>
#include <stdint.h>

/**
 * The various error codes used by this library.
 *
 * Every non-trivial function should return a wasm_result_t to indicate
 * whether it executed successfully.
 */
enum wasm_result_t {
  // The operation was successful.
  WASM_SUCCESS = 0,
  // An unspecified error occurred.
  WASM_GENERIC_ERROR = 1,
  // Tried to access an input/output address which is out of bounds.
  WASM_ADDRESS_OUT_OF_BOUNDS = 2,
  // Tried to read an unknown variable.
  WASM_UNKNOWN_VARIABLE = 3,
  // Tried to read/write a variable using the wrong type (e.g. you tried to
  // write a boolean to an integer variable).
  WASM_BAD_VARIABLE_TYPE = 4,
};

/**
 * The log levels used with `wasm_log()`.
 */
enum wasm_log_level {
  LOG_ERROR = 0,
  LOG_WARN = 1,
  LOG_INFO = 2,
  LOG_DEBUG = 3,
  LOG_TRACE = 4,
};

/**
 * Log a message at the specified level, including information about the file
 * and line the message was logged from.
 */
enum wasm_result_t wasm_log(enum wasm_log_level level, const char *file, int file_len, int line,
             const char *message, int message_len);

/**
 * Convenience macro for logging a message.
 */
#define LOG(level, message) wasm_log(level, __FILE__, strlen(__FILE__), __LINE__, message, strlen(message))

/**
 * Read from an input from memory-mapped IO.
 */
enum wasm_result_t wasm_read_input(uint32_t address, char *buffer, int buffer_len);

/**
 * Write to an output using memory-mapped IO.
 */
enum wasm_result_t wasm_write_output(uint32_t address, const char *data, int data_len);

/**
 * Get a measurement of a monotonically nondecreasing clock.
 *
 * The absolute numbers don't necessarily mean anything, the difference
 * between two measurements can be used to tell how much time has passed.
 */
enum wasm_result_t wasm_current_time(uint64_t *secs, uint32_t *nanos);

/**
 * Read a globally defined boolean variable.
 *
 * Reading an unknown variable or trying to access a variable using the wrong
 * type will result in an error.
 */
enum wasm_result_t wasm_variable_read_boolean(const char *name, int name_len, bool *value);

/**
 * Read a globally defined floating-point variable.
 *
 * Reading an unknown variable or trying to access a variable using the wrong
 * type will result in an error.
 */
enum wasm_result_t wasm_variable_read_double(const char *name, int name_len, double *value);

/**
 * Read a globally defined integer variable.
 *
 * Reading an unknown variable or trying to access a variable using the wrong
 * type will result in an error.
 */
enum wasm_result_t wasm_variable_read_int(const char *name, int name_len, int32_t *value);

/**
 * Write to a globally defined boolean variable.
 *
 * This may fail if the variable already exists and has a different type.
 */
enum wasm_result_t wasm_variable_write_boolean(const char *name, int name_len, bool value);

/**
 * Write to a globally defined floating-point variable.
 *
 * This may fail if the variable already exists and has a different type.
 */
enum wasm_result_t wasm_variable_write_double(const char *name, int name_len, double value);

/**
 * Write to a globally defined integer variable.
 *
 * This may fail if the variable already exists and has a different type.
 */
enum wasm_result_t wasm_variable_write_int(const char *name, int name_len, int32_t value);

#endif // WASM_INTRINSICS_H