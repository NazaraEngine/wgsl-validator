#include <stdio.h>
#include "../ffi/wgsl_validator.h"

#define WGSL_SOURCE(...) #__VA_ARGS__

const char* wgsl_source = WGSL_SOURCE(
@fragment
fn main_fs() -> @location(0) vec4<f32> {
	return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
);

int main(void)
{
	char* error;
	wgsl_validator_t* validator = wgsl_validator_create();
	if(wgsl_validator_validate(validator, wgsl_source, &error))
	{
		fprintf(stderr, "%s\n", error);
		wgsl_validator_free_error(error);
		return 1;
	}
	wgsl_validator_destroy(validator);
	puts("Successfully parsed shader");
	return 0;
}
