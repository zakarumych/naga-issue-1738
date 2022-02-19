const SHADER_CODE: &'static str = r"
    struct VertexInput {
        [[location(0)]] pos: vec3<f32>;
    };

    struct VertexOutput {
        [[builtin(position)]] pos: vec4<f32>;
    };

    struct Uniforms {
        albedo_factor: vec4<f32>;
        camera_view: mat4x4<f32>;
        camera_proj: mat4x4<f32>;
        transform: mat4x4<f32>;
    };

    [[group(0), binding(2)]]
    var uniforms: Uniforms;

    [[stage(vertex)]]
    fn vs_main(
        in: VertexInput,
    ) -> VertexOutput {
        var out: VertexOutput;

        out.pos = uniforms.camera_proj * uniforms.camera_view * uniforms.transform * vec4<f32>(in.pos, 1.0);

        return out;
    }

    [[stage(fragment)]]
    fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }
";

fn main() {
    let module = naga::front::wgsl::parse_str(SHADER_CODE).unwrap();
    let info = naga::valid::Validator::new(naga::valid::ValidationFlags::all(), Default::default())
        .validate(&module)
        .unwrap();

    // Unreachable here.
    let spv =
        naga::back::spv::write_vec(&module, &info, &naga::back::spv::Options::default(), None)
            .unwrap();
}
