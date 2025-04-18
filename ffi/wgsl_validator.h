#ifndef WGSL_VALIDATOR_H
#define WGSL_VALIDATOR_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct wgsl_validator wgsl_validator_t;

wgsl_validator_t* wgsl_validator_create();
void wgsl_validator_destroy(wgsl_validator_t* ptr);

// Validates a WGSL shader string. Returns 0 on success, non-zero on failure.
// On failure, out_error will point to a malloc'd C string. Caller must free it with `wgsl_validator_free_error`.
int wgsl_validator_validate(wgsl_validator_t* validator, const char* shader_src, char** out_error);

void wgsl_validator_free_error(char* err_str);

#ifdef __cplusplus
}
#endif

#endif // WGSL_VALIDATOR_H
