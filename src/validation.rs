use naga::{
    front::wgsl,
    valid::{Capabilities, ValidationFlags},
};

use crate::errors::WgslError;

pub struct Validator {
    validator: naga::valid::Validator,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            validator: naga::valid::Validator::new(ValidationFlags::all(), Capabilities::all()),
        }
    }

    pub fn validate_wgsl(&mut self, shader: &str) -> Result<(), WgslError> {
        let module =
            wgsl::parse_str(shader).map_err(|err| WgslError::from_parse_err(err, shader))?;

        if let Err(error) = self.validator.validate(&module) {
            Err(WgslError::ValidationErr {
                emitted: error.emit_to_string(shader),
                src: shader.to_string(),
                error,
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_validation() -> Result<(), WgslError> {
        let wgsl_source = r#"
            @fragment
            fn main_fs() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 1.0, 1.0, 1.0);
            }
        "#;
        let mut validator = Validator::new();
        validator.validate_wgsl(wgsl_source)
    }

    #[test]
    fn incorrect_parsing() {
        let wgsl_source = r#"
            @invalid_entry
            fn main_fs() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 1.0, 1.0, 1.0);
            }
        "#;
        let mut validator = Validator::new();
        assert!(validator.validate_wgsl(wgsl_source).is_err());
    }

    #[test]
    fn incorrect_validation() {
        let wgsl_source = r#"
            @fragment
            fn main_fs() -> @location(0) vec2<f32> {
                return vec4<f32>(1.0, 1.0, 1.0, 1.0);
            }
        "#;
        let mut validator = Validator::new();
        assert!(validator.validate_wgsl(wgsl_source).is_err());
    }

    #[test]
    fn correct_complex_validation() -> Result<(), WgslError> {
        let wgsl_source = r#"
            struct UBO
            {
                screen_size: vec2<f32>,
                center: vec2<f32>,
                scale: f32,
                iteration_count: i32
            }

            @group(0) @binding(0) var palette: texture_2d<f32>;
            @group(0) @binding(1) var paletteSampler: sampler;
            @group(0) @binding(2) var<uniform> ubo: UBO;

            struct Input
            {
                @builtin(position) fragcoord: vec4<f32>
            }

            struct Output
            {
                @location(0) color: vec4<f32>
            }

            @fragment
            fn main(input: Input) -> Output
            {
                var coords: vec2<f32> = input.fragcoord.xy / ubo.screen_size;
                var c: vec2<f32>;
                c.x = (((ubo.screen_size.x / ubo.screen_size.y) * (coords.x - (0.5))) * ubo.scale) - (ubo.center.x / ubo.screen_size.y);
                c.y = ((coords.y - (0.5)) * ubo.scale) - (ubo.center.y / ubo.screen_size.y);
                var z: vec2<f32> = c;
                var i: i32 = 0;
                while (i < ubo.iteration_count)
                {
                    var x: f32 = ((z.x * z.x) - (z.y * z.y)) + c.x;
                    var y: f32 = ((z.y * z.x) + (z.x * z.y)) + c.y;
                    if (((x * x) + (y * y)) > (4.0))
                    {
                        break;
                    }

                    z.x = x;
                    z.y = y;
                    i += 1;
                }

                var u: f32;
                if (i < ubo.iteration_count)
                {
                    u = (f32(i)) / (100.0);
                }
                else
                {
                    u = 0.0;
                }

                var output: Output;
                output.color = textureSample(palette, paletteSampler, vec2<f32>(u, 0.0));
                return output;
            }
        "#;
        let mut validator = Validator::new();
        validator.validate_wgsl(wgsl_source)
    }
}
