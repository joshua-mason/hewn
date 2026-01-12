// =============================================================================
// HEWN SPRITE SHADER
// =============================================================================
// This shader renders textured sprites (quads) from a tilemap.
//
// It uses instancing to efficiently render many sprites in one draw call:
// - Each sprite shares the same quad geometry (4 vertices)
// - Each instance has its own position, color tint, and tile index
// - The tile index selects which sprite from the tilemap to render
// - If tile_index == 4294967295 (u32::MAX), render solid color instead
// =============================================================================

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------
// Tilemap dimensions - UPDATE THESE if using a different tilemap!
const TILEMAP_TILES_X: u32 = 20u;
const TILEMAP_TILES_Y: u32 = 20u;
const TILE_SIZE_U: f32 = 0.05;  // 1.0 / 20.0
const TILE_SIZE_V: f32 = 0.05;  // 1.0 / 20.0

// Special value meaning "no texture, use solid color"
const NO_TEXTURE: u32 = 4294967295u;

// -----------------------------------------------------------------------------
// Instance Data (per-sprite)
// -----------------------------------------------------------------------------
// Each sprite instance provides a 4x4 transformation matrix for positioning.
struct InstancePositionInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

// Per-instance color tint (RGB)
struct InstanceColorInput {
    @location(9) color: vec3<f32>,
};

// Per-instance tile index in the tilemap
struct InstanceTileInput {
    @location(10) tile_index: u32,
};

// -----------------------------------------------------------------------------
// Camera (shared by all sprites)
// -----------------------------------------------------------------------------
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

// -----------------------------------------------------------------------------
// Vertex Shader Input/Output
// -----------------------------------------------------------------------------
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) @interpolate(flat) tile_index: u32,  // flat = no interpolation
};

// -----------------------------------------------------------------------------
// Texture Bindings
// -----------------------------------------------------------------------------
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// -----------------------------------------------------------------------------
// Vertex Shader
// -----------------------------------------------------------------------------
@vertex
fn vs_main(
    model: VertexInput,
    instance: InstancePositionInput,
    instance_color: InstanceColorInput,
    instance_tile: InstanceTileInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Reconstruct the 4x4 model matrix from the 4 vec4 columns
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );
    
    // Transform: model space -> world space -> clip space
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    
    // Pass through data to fragment shader
    out.tex_coords = model.tex_coords;
    out.color = instance_color.color;
    out.tile_index = instance_tile.tile_index;
    
    return out;
}

// -----------------------------------------------------------------------------
// Fragment Shader
// -----------------------------------------------------------------------------
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Check if we should render solid color (no texture)
    if (in.tile_index == NO_TEXTURE) {
        return vec4<f32>(in.color, 1.0);
    }
    
    // Calculate which tile in the tilemap (row-major order)
    let tile_x = in.tile_index % TILEMAP_TILES_X;
    let tile_y = in.tile_index / TILEMAP_TILES_X;
    
    // Calculate UV offset for this tile
    // tile_x=0, tile_y=0 is top-left of tilemap
    let uv_offset = vec2<f32>(
        f32(tile_x) * TILE_SIZE_U,
        f32(tile_y) * TILE_SIZE_V
    );
    
    // Scale the vertex UVs (0-1) to tile size and add offset
    // in.tex_coords are the UVs for the quad (0-1 range)
    let tile_uv = uv_offset + (in.tex_coords * vec2<f32>(TILE_SIZE_U, TILE_SIZE_V));
    
    // Sample the tilemap at the calculated UV
    let tex_color = textureSample(t_diffuse, s_diffuse, tile_uv);
    
    // Discard fully transparent pixels (for transparent tilemap)
    if (tex_color.a < 0.1) {
        discard;
    }
    
    // Return texture color (could multiply by in.color for tinting)
    return tex_color;
}
