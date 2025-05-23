#version 450
#include <vulkano.glsl>
#include <shared_exponent.glsl>

layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

layout(push_constant) uniform PushConstants {
    SamplerId sampler_id;
    SampledImageId texture_id;
    StorageImageId dst_mip_image_id;
    uint dst_mip_level;
    float intensity;
};

uint src_mip_level = dst_mip_level + 1;

vec3 sample1(vec2 uv) {
    return textureLod(vko_sampler2D(texture_id, sampler_id), uv, src_mip_level).rgb;
}

// 9-tap tent filter.
vec3 upsampleTent9(vec2 uv, vec2 src_texel_size) {
    vec3 color;
    color  = sample1(uv + vec2(-1.0, -1.0) * src_texel_size) * 1.0;
    color += sample1(uv + vec2( 0.0, -1.0) * src_texel_size) * 2.0;
    color += sample1(uv + vec2( 1.0, -1.0) * src_texel_size) * 1.0;
    color += sample1(uv + vec2(-1.0,  0.0) * src_texel_size) * 2.0;
    color += sample1(uv + vec2( 0.0,  0.0) * src_texel_size) * 4.0;
    color += sample1(uv + vec2( 1.0,  0.0) * src_texel_size) * 2.0;
    color += sample1(uv + vec2(-1.0,  1.0) * src_texel_size) * 1.0;
    color += sample1(uv + vec2( 0.0,  1.0) * src_texel_size) * 2.0;
    color += sample1(uv + vec2( 1.0,  1.0) * src_texel_size) * 1.0;

    return color * (1.0 / 16.0);
}

void blend(vec2 uv, ivec2 dst_coord, vec3 color) {
    color += textureLod(vko_sampler2D(texture_id, sampler_id), uv, dst_mip_level).rgb;
    uint packed = convertToSharedExponent(color);
    imageStore(vko_uimage2D_r32ui(dst_mip_image_id), dst_coord, uvec4(packed, 0, 0, 0));
}

void main() {
    ivec2 dst_coord = ivec2(gl_GlobalInvocationID.xy);
    ivec2 dst_size = imageSize(vko_uimage2D_r32ui(dst_mip_image_id));

    if (dst_coord.x > dst_size.x || dst_coord.y > dst_size.y) {
        return;
    }

    ivec2 src_size = textureSize(vko_texture2D(texture_id), int(src_mip_level));
    vec2 src_texel_size = 1.0 / vec2(src_size);
    vec2 uv = (vec2(dst_coord) + 0.5) / vec2(dst_size);
    vec3 color = upsampleTent9(uv, src_texel_size);

    color *= intensity;

    blend(uv, dst_coord, color);
}
