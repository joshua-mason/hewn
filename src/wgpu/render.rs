use crate::scene::Entity;
use crate::scene::EntityId;
use crate::wgpu::texture;
use cgmath::prelude::*;
use cgmath::SquareMatrix;
use image;
use std::mem;
use std::{iter, sync::Arc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

#[derive(Default, Copy, Clone)]
pub enum CameraStrategy {
    #[default]
    AllEntities,
    CameraFollow(EntityId),
}

/// Configuration for a tilemap (sprite sheet) used for rendering.
///
/// A tilemap is a single image containing multiple sprites arranged in a grid.
/// Each sprite can be referenced by its tile index (0-indexed, row-major order).
///
/// # Example
/// ```ignore
/// // A 20x20 tilemap with 400 tiles total
/// let tilemap = Tilemap::new(include_bytes!("my_tilemap.png"), 20, 20);
/// // Tile 0 is top-left, tile 19 is top-right, tile 20 is second row left, etc.
/// ```
/// Note: Clone and Copy are derived because bytes is `&'static` (compile-time embedded)
#[derive(Clone, Copy)]
pub struct Tilemap {
    /// The raw PNG/JPEG bytes of the tilemap image
    pub bytes: &'static [u8],
    /// Number of tiles horizontally
    pub tiles_x: u32,
    /// Number of tiles vertically
    pub tiles_y: u32,
}

impl Tilemap {
    /// Creates a new tilemap configuration.
    pub fn new(bytes: &'static [u8], tiles_x: u32, tiles_y: u32) -> Self {
        Self {
            bytes,
            tiles_x,
            tiles_y,
        }
    }

    /// Returns the total number of tiles in this tilemap.
    pub fn total_tiles(&self) -> u32 {
        self.tiles_x * self.tiles_y
    }
}

/// A vertex with position and texture coordinates.
///
/// - `position`: The 3D position of this vertex in model space (z is always 0 for 2D sprites)
/// - `tex_coords`: UV coordinates for texture mapping (0,0 is top-left, 1,1 is bottom-right)
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

/// Generates a textured quad (rectangle) for sprite rendering.
///
/// Returns 4 vertices forming a square centered at origin, with proper UV coordinates
/// so that a texture maps correctly onto it.
///
/// UV coordinate layout:
/// ```text
///   (0,0)-----(1,0)
///     |         |
///     |  IMAGE  |
///     |         |
///   (0,1)-----(1,1)
/// ```
fn gen_textured_quad(size: f32) -> (Vec<Vertex>, Vec<u16>) {
    let half = size / 2.0;

    // Four corners of a quad, with UV coordinates mapping the full texture
    let vertices = vec![
        // Bottom-left vertex
        Vertex {
            position: [-half, -half, 0.0],
            tex_coords: [0.0, 1.0], // Bottom-left of texture
        },
        // Bottom-right vertex
        Vertex {
            position: [half, -half, 0.0],
            tex_coords: [1.0, 1.0], // Bottom-right of texture
        },
        // Top-right vertex
        Vertex {
            position: [half, half, 0.0],
            tex_coords: [1.0, 0.0], // Top-right of texture
        },
        // Top-left vertex
        Vertex {
            position: [-half, half, 0.0],
            tex_coords: [0.0, 0.0], // Top-left of texture
        },
    ];

    // Two triangles forming the quad:
    // Triangle 1: bottom-left, bottom-right, top-right (0, 1, 2)
    // Triangle 2: bottom-left, top-right, top-left (0, 2, 3)
    let indices = vec![0, 1, 2, 0, 2, 3];

    (vertices, indices)
}

impl Vertex {
    /// Describes the memory layout of our Vertex struct for the GPU.
    ///
    /// This tells wgpu how to read vertex data from our buffer:
    /// - Location 0: position (3 floats = 12 bytes)
    /// - Location 1: tex_coords (2 floats = 8 bytes)
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            // Total size of one Vertex struct (position + tex_coords = 20 bytes)
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position attribute at shader location 0
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Texture coordinates at shader location 1
                // Offset is after the position (3 floats = 12 bytes)
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub(crate) struct Camera {
    pub(crate) eye: cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up: cgmath::Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

impl Camera {
    pub(crate) fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct CameraUniform {
    pub(crate) view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into();
    }
}

pub(crate) struct InstancePosition {
    pub(crate) position: cgmath::Vector3<f32>,
    pub(crate) rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct InstancePositionRaw {
    pub(crate) model: [[f32; 4]; 4],
}

impl InstancePositionRaw {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstancePositionRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl InstancePosition {
    pub(crate) fn to_raw(&self) -> InstancePositionRaw {
        InstancePositionRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}

pub(crate) struct InstanceColor {
    pub(crate) color: cgmath::Vector3<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct InstanceColorRaw {
    pub(crate) model: [f32; 3],
}

impl InstanceColorRaw {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceColorRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 9,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

impl InstanceColor {
    pub(crate) fn to_raw(&self) -> InstanceColorRaw {
        InstanceColorRaw {
            model: self.color.into(),
        }
    }
}

/// Per-instance tile index for tilemap rendering.
///
/// - `tile_index`: The tile index in the tilemap (0-indexed, row-major order)
///                 Use u32::MAX to indicate "no texture, use solid color"
pub(crate) struct InstanceTile {
    pub(crate) tile_index: u32,
}

/// Value indicating no texture should be used (render solid color instead)
pub const NO_TEXTURE_TILE: u32 = u32::MAX;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct InstanceTileRaw {
    pub(crate) tile_index: u32,
}

impl InstanceTileRaw {
    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceTileRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 10, // After color at location 9
                format: wgpu::VertexFormat::Uint32,
            }],
        }
    }
}

impl InstanceTile {
    pub(crate) fn to_raw(&self) -> InstanceTileRaw {
        InstanceTileRaw {
            tile_index: self.tile_index,
        }
    }
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    #[allow(dead_code)]
    diffuse_texture: texture::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    instance_positions: Vec<InstancePosition>,
    instance_colors: Vec<InstanceColor>,
    instance_tiles: Vec<InstanceTile>,
    instance_positions_buffer: wgpu::Buffer,
    instance_colors_buffer: wgpu::Buffer,
    instance_tiles_buffer: wgpu::Buffer,
    camera_strategy: CameraStrategy,

    /// Tilemap dimensions (tiles_x, tiles_y) - needed for UV calculation in shader
    tilemap_tiles_x: u32,
    tilemap_tiles_y: u32,

    vertices: Vec<Vertex>,
    indices: Vec<u16>,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub(crate) window: Arc<Window>,
    renderable_entities: Vec<Entity>,
}

impl State {
    pub(crate) async fn new(
        window: Arc<Window>,
        renderable_entities: Vec<Entity>,
        camera_strategy: CameraStrategy,
        tilemap: Option<Tilemap>,
    ) -> anyhow::Result<State> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Load tilemap texture if provided, otherwise use a default 1x1 white texture
        let (diffuse_texture, tilemap_tiles_x, tilemap_tiles_y) = if let Some(ref tm) = tilemap {
            let texture =
                texture::Texture::from_bytes(&device, &queue, tm.bytes, "tilemap").unwrap();
            (texture, tm.tiles_x, tm.tiles_y)
        } else {
            // Default: 1x1 white texture for solid color rendering
            let white_pixel: [u8; 4] = [255, 255, 255, 255];
            let img = image::RgbaImage::from_raw(1, 1, white_pixel.to_vec()).unwrap();
            let dyn_img = image::DynamicImage::ImageRgba8(img);
            let texture =
                texture::Texture::from_image(&device, &queue, &dyn_img, Some("default_white"))
                    .unwrap();
            (texture, 1, 1)
        };

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let camera = Camera {
            eye: (0.0, 1.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    Vertex::desc(),
                    InstancePositionRaw::desc(),
                    InstanceColorRaw::desc(),
                    InstanceTileRaw::desc(),
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });

        // Generate a textured quad for sprite rendering
        // Size 0.16 gives a reasonable sprite size in world units
        let (vertices, indices) = gen_textured_quad(0.12);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = indices.len() as u32;

        // Create per-instance data from entities
        let (instance_positions, instance_colors, instance_tiles): (
            Vec<InstancePosition>,
            Vec<InstanceColor>,
            Vec<InstanceTile>,
        ) = renderable_entities
            .iter()
            .map(|e| {
                let position = e.components.position.unwrap();
                let render = e.components.render.unwrap();
                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                );

                let position_3d = cgmath::Vector3 {
                    x: position.x as f32 * 0.1,
                    y: position.y as f32 * 0.1,
                    z: 0.0,
                };

                // Get tile index, or NO_TEXTURE_TILE if no sprite is set
                let tile_index = render
                    .sprite_tile
                    .map(|t| t as u32)
                    .unwrap_or(NO_TEXTURE_TILE);

                (
                    InstancePosition {
                        position: position_3d,
                        rotation,
                    },
                    InstanceColor { color: render.rgb },
                    InstanceTile { tile_index },
                )
            })
            .fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut positions, mut colors, mut tiles), (p, c, t)| {
                    positions.push(p);
                    colors.push(c);
                    tiles.push(t);
                    (positions, colors, tiles)
                },
            );

        let instance_positions_raw = instance_positions
            .iter()
            .map(InstancePosition::to_raw)
            .collect::<Vec<_>>();
        let instance_colors_raw = instance_colors
            .iter()
            .map(InstanceColor::to_raw)
            .collect::<Vec<_>>();
        let instance_tiles_raw = instance_tiles
            .iter()
            .map(InstanceTile::to_raw)
            .collect::<Vec<_>>();
        let instance_positions_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Position Buffer"),
                contents: bytemuck::cast_slice(&instance_positions_raw),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let instance_colors_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Color Buffer"),
            contents: bytemuck::cast_slice(&instance_colors_raw),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let instance_tiles_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Tile Buffer"),
            contents: bytemuck::cast_slice(&instance_tiles_raw),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_texture,
            diffuse_bind_group,
            vertices,
            indices,
            camera,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instance_positions,
            instance_colors,
            instance_tiles,
            instance_positions_buffer,
            instance_colors_buffer,
            instance_tiles_buffer,
            window,
            renderable_entities,
            camera_strategy,
            tilemap_tiles_x,
            tilemap_tiles_y,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.is_surface_configured = true;
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);

            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
    }

    pub(crate) fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: KeyCode, pressed: bool) {
        if key == KeyCode::Escape && pressed {
            event_loop.exit();
        }
    }

    pub(crate) fn update(&mut self, renderable_entities: Vec<Entity>) {
        self.renderable_entities = renderable_entities;

        let (instance_positions, instance_colors, instance_tiles): (
            Vec<InstancePosition>,
            Vec<InstanceColor>,
            Vec<InstanceTile>,
        ) = self
            .renderable_entities
            .iter()
            .map(|e| {
                let position = e.components.position.unwrap();
                let render = e.components.render.unwrap();
                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_z(),
                    cgmath::Deg(0.0),
                );

                let position_3d = cgmath::Vector3 {
                    x: position.x as f32 * 0.1,
                    y: position.y as f32 * 0.1,
                    z: 0.0,
                };

                // Get tile index, or NO_TEXTURE_TILE if no sprite is set
                let tile_index = render
                    .sprite_tile
                    .map(|t| t as u32)
                    .unwrap_or(NO_TEXTURE_TILE);

                (
                    InstancePosition {
                        position: position_3d,
                        rotation,
                    },
                    InstanceColor { color: render.rgb },
                    InstanceTile { tile_index },
                )
            })
            .fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut positions, mut colors, mut tiles), (p, c, t)| {
                    positions.push(p);
                    colors.push(c);
                    tiles.push(t);
                    (positions, colors, tiles)
                },
            );
        self.instance_positions = instance_positions;
        self.instance_colors = instance_colors;
        self.instance_tiles = instance_tiles;
        let instance_position_data = self
            .instance_positions
            .iter()
            .map(InstancePosition::to_raw)
            .collect::<Vec<_>>();
        let instance_positions_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Position Buffer"),
                    contents: bytemuck::cast_slice(&instance_position_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let instance_colors_data = self
            .instance_colors
            .iter()
            .map(InstanceColor::to_raw)
            .collect::<Vec<_>>();
        let instance_colors_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Position Buffer"),
                    contents: bytemuck::cast_slice(&instance_colors_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let instance_tiles_data = self
            .instance_tiles
            .iter()
            .map(InstanceTile::to_raw)
            .collect::<Vec<_>>();
        let instance_tiles_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Tile Buffer"),
                    contents: bytemuck::cast_slice(&instance_tiles_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        // Log the instance positions to stdout so we can see things are moving when keys are hit.
        self.instance_positions_buffer = instance_positions_buffer;
        self.instance_colors_buffer = instance_colors_buffer;
        self.instance_tiles_buffer = instance_tiles_buffer;

        let camera_points =
            self.renderable_entities
                .iter()
                .fold((0.0, 0.0, 0.0, 0.0), |mut acc, e| {
                    if let Some(position) = e.components.position {
                        if position.x < acc.0 {
                            acc.0 = position.x;
                        }
                        if position.x > acc.1 {
                            acc.1 = position.x;
                        }
                        if position.y < acc.2 {
                            acc.2 = position.y;
                        }
                        if position.y > acc.3 {
                            acc.3 = position.y;
                        }
                    }
                    acc
                });
        let camera_x_position = (camera_points.0 + camera_points.1) / 2.0;
        let camera_y_position = (1.0 - camera_points.2 + camera_points.3) / 2.0;

        match self.camera_strategy {
            CameraStrategy::CameraFollow(entity_id) => {
                let entity = self
                    .renderable_entities
                    .iter()
                    .find(|e| e.id == entity_id)
                    .unwrap(); // what do we do in the case the entity doesn't exist?
                let camera_follow_position = entity.components.position.unwrap();
                self.camera.eye = cgmath::Point3::new(
                    camera_follow_position.x as f32 * 0.1,
                    camera_follow_position.y as f32 * 0.1,
                    4.0,
                );
                self.camera.target = cgmath::Point3::new(
                    camera_follow_position.x as f32 * 0.1,
                    camera_follow_position.y as f32 * 0.1,
                    0.0,
                );
                self.camera_uniform.update_view_proj(&self.camera);
            }
            CameraStrategy::AllEntities => {
                let game_width = camera_points.1 - camera_points.0;
                let z_depth = game_width as f32 / 8.1;
                self.camera.eye =
                    cgmath::Point3::new(camera_x_position * 0.1, camera_y_position * 0.1, z_depth);
                self.camera.target =
                    cgmath::Point3::new(camera_x_position * 0.1, camera_y_position * 0.1, 0.0);
                self.camera_uniform.update_view_proj(&self.camera);
            }
        }

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_positions_buffer.slice(..));
            render_pass.set_vertex_buffer(2, self.instance_colors_buffer.slice(..));
            render_pass.set_vertex_buffer(3, self.instance_tiles_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(
                0..self.num_indices,
                0,
                0..self.instance_positions.len() as _,
            );
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
