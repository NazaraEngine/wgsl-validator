# WGSL Validator

A simple WGSL validator in Rust over [naga](https://crates.io/crates/naga) with C bindings.

```rust
use wgsl_validator::*;

fn main() -> Result<(), WgslError> {
    let wgsl_source = r#"
        @fragment
        fn main_fs() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 1.0, 1.0, 1.0);
        }
    "#;
    let mut validator = Validator::new();
    validator.validate_wgsl(wgsl_source)
}
```

Or using the C API

```c
#include <stdio.h>
#include "wgsl_validator.h"

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
	}
	wgsl_validator_destroy(validator);
	return 0;
}
```
