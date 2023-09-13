/* WARNING: this file was automatically generated. Do not edit. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * a test function,should print "Hello, world!"
 */
void rust_utils_hello(void);

/**
 * clear the saved tensors
 */
void rust_utils_clear(void);

/**
 * print the saved tensors
 */
void rust_utils_print(void);

/**
 * add a tensor to the saved tensors, the data are ggml format
 * - `name` is the name of the tensor, should be valid c/c++ string
 * - `data` is the data of the tensor, should be a pointer to a f32 array in ggml format
 */
void rust_utils_add_element(const char *name,
                            const float *data,
                            const int64_t (*ne)[4],
                            const uint64_t (*nb)[4]);

/**
 * save the saved tensors to a file
 */
void rust_utils_save_elements(const char *path);

/**
 * load from a file
 */
void rust_utils_load_elements(const char *path);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
