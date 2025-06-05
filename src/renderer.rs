use egui_wgpu::wgpu;
use crate::model::enums::ParticleShape;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2, // position
        1 => Float32x4, // color
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct Renderer {
    // Particle rendering pipeline
    particle_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    num_vertices: u32,
    
    // Layer textures
    background_texture: wgpu::Texture,
    background_view: wgpu::TextureView,
    vector_texture: wgpu::Texture,
    vector_view: wgpu::TextureView,
    particle_texture: wgpu::Texture,
    particle_view: wgpu::TextureView,
    
    // Layer bind groups
    background_bind_group: wgpu::BindGroup,
    vector_bind_group: wgpu::BindGroup,
    particle_bind_group: wgpu::BindGroup,
    particle_texture_bind_group: wgpu::BindGroup,
    
    // MSAA textures
    msaa_texture: wgpu::Texture,
    msaa_view: wgpu::TextureView,
    resolve_texture: wgpu::Texture,
    resolve_view: wgpu::TextureView,
    
    // Background quad rendering
    quad_pipeline: wgpu::RenderPipeline,

    // Vector field rendering
    vector_pipeline: wgpu::RenderPipeline,
    vector_buffer: wgpu::Buffer,
    num_vector_vertices: u32,

    // Bind group layouts
    background_bind_group_layout: wgpu::BindGroupLayout,
    vector_bind_group_layout: wgpu::BindGroupLayout,
    particle_bind_group_layout: wgpu::BindGroupLayout,
    particle_texture_bind_group_layout: wgpu::BindGroupLayout,

    // Size
    size: (u32, u32),

    particle_uniform_buffer: wgpu::Buffer,
}

impl Renderer {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, size: (u32, u32)) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Particle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });
        
        let quad_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Quad Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/quad.wgsl").into()),
        });

        // Create uniform buffer for resolution
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform_bind_group_layout"),
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
        });

        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create texture bind group layouts
        let background_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Background Bind Group Layout"),
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
        });

        let vector_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Vector Bind Group Layout"),
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
        });

        let particle_texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Particle Texture Bind Group Layout"),
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
        });

        // Create quad pipeline layout
        let quad_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Quad Pipeline Layout"),
            bind_group_layouts: &[
                &background_bind_group_layout,
                &vector_bind_group_layout,
                &particle_texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        let quad_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Pipeline"),
            layout: Some(&quad_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &quad_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &quad_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Particle pipeline bind group layout (for uniforms)
        let particle_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Particle Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let particle_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Particle Pipeline Layout"),
            bind_group_layouts: &[&particle_bind_group_layout],
            push_constant_ranges: &[],
        });
        let particle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Particle Pipeline"),
            layout: Some(&particle_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<[f32; 6]>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as u64,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Create MSAA texture for supersampling
        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4, // 4x MSAA
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create resolve texture for MSAA resolution
        let resolve_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Resolve Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        let resolve_view = resolve_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create sampler for resolve texture
        let resolve_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Resolve Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group for resolve texture
        let resolve_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Resolve Bind Group Layout"),
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
        });

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Resolve Bind Group"),
            layout: &resolve_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolve_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&resolve_sampler),
                },
            ],
        });
        
        // Create sampler for trail texture
        let trail_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Trail Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        
        // Create bind group layout for quad rendering
        let quad_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Quad Bind Group Layout"),
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
        });
        
       device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Quad Bind Group"),
            layout: &quad_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&resolve_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&trail_sampler),
                },
            ],
        });
        
        // Create vector pipeline
        let vector_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Vector Pipeline"),
            layout: Some(&particle_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Get device limits
        let max_buffer_size = device.limits().max_buffer_size;

        // Create empty vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create empty vector buffer
        let vector_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vector Buffer"),
            size: max_buffer_size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });


        // Create layer textures
        let background_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Background Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        let background_view = background_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let vector_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Vector Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        let vector_view = vector_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let particle_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Particle Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        let particle_view = particle_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create samplers for each layer
        let background_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Background Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let vector_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Vector Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let particle_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Particle Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind groups for each layer
        let background_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Background Bind Group"),
            layout: &background_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&background_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&background_sampler),
                },
            ],
        });

        let vector_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Vector Bind Group"),
            layout: &vector_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&vector_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&vector_sampler),
                },
            ],
        });


        // Create particle uniform buffer
        let particle_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Particle Uniform Buffer"),
            size: std::mem::size_of::<[f32; 2]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // For the quad pipeline (compositing pass)
        let particle_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Texture Bind Group"),
            layout: &particle_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&particle_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&particle_sampler),
                },
            ],
        });

        // For the particle pipeline (particle pass)
        let particle_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Bind Group"),
            layout: &particle_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        // Fallback: create a 1x1 transparent texture and bind group for the vector layer if needed
        let fallback_vector_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Fallback Vector Texture"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let fallback_vector_view = fallback_vector_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let fallback_vector_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Fallback Vector Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Fallback Vector Bind Group"),
            layout: &vector_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&fallback_vector_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&fallback_vector_sampler),
                },
            ],
        });

        // Create egui offscreen texture
        let egui_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Egui Texture"),
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let egui_view = egui_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let egui_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Egui Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let egui_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Egui Bind Group Layout"),
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
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Egui Bind Group"),
            layout: &egui_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&egui_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&egui_sampler),
                },
            ],
        });

        Self {
            particle_pipeline,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            num_vertices: 0,
            background_texture,
            background_view,
            vector_texture,
            vector_view,
            particle_texture,
            particle_view,
            msaa_texture,
            msaa_view,
            resolve_texture,
            resolve_view,
            quad_pipeline,
            vector_pipeline,
            vector_buffer,
            num_vector_vertices: 0,
            background_bind_group,
            vector_bind_group,
            particle_bind_group,
            particle_texture_bind_group,
            background_bind_group_layout,
            vector_bind_group_layout,
            particle_bind_group_layout,
            particle_texture_bind_group_layout,
            size,
            particle_uniform_buffer,
        }
    }

    fn generate_shape_vertices(
        &self,
        center_x: f32,
        center_y: f32,
        size: f32,
        color: [f32; 4],
        shape: ParticleShape,
    ) -> Vec<Vertex> {
        match shape {
            ParticleShape::Circle => {
                // Generate a circle using multiple triangles
                let segments = 16;
                let mut vertices = Vec::with_capacity(segments * 3);
                let angle_step = std::f32::consts::TAU / segments as f32;
                
                for i in 0..segments {
                    let angle1 = i as f32 * angle_step;
                    let angle2 = (i + 1) as f32 * angle_step;
                    
                    let x1 = center_x + size * angle1.cos();
                    let y1 = center_y + size * angle1.sin();
                    let x2 = center_x + size * angle2.cos();
                    let y2 = center_y + size * angle2.sin();
                    
                    vertices.push(Vertex { position: [center_x, center_y], color });
                    vertices.push(Vertex { position: [x1, y1], color });
                    vertices.push(Vertex { position: [x2, y2], color });
                }
                vertices
            }
            ParticleShape::Square => {
                let half_size = size * 0.5;
                vec![
                    Vertex { position: [center_x - half_size, center_y - half_size], color },
                    Vertex { position: [center_x + half_size, center_y - half_size], color },
                    Vertex { position: [center_x - half_size, center_y + half_size], color },
                    Vertex { position: [center_x + half_size, center_y - half_size], color },
                    Vertex { position: [center_x + half_size, center_y + half_size], color },
                    Vertex { position: [center_x - half_size, center_y + half_size], color },
                ]
            }
            ParticleShape::Triangle => {
                let height = size * 0.866; // sqrt(3)/2
                vec![
                    Vertex { position: [center_x, center_y - size], color },
                    Vertex { position: [center_x - size, center_y + height], color },
                    Vertex { position: [center_x + size, center_y + height], color },
                ]
            }
            ParticleShape::Star => {
                let mut vertices = Vec::with_capacity(15);
                let outer_radius = size;
                let inner_radius = size * 0.4;
                let points = 5;
                let angle_step = std::f32::consts::TAU / points as f32;
                
                for i in 0..points {
                    let angle1 = i as f32 * angle_step;
                    let angle2 = angle1 + angle_step * 0.5;
                    
                    // Outer point
                    let x1 = center_x + outer_radius * angle1.cos();
                    let y1 = center_y + outer_radius * angle1.sin();
                    
                    // Inner point
                    let x2 = center_x + inner_radius * angle2.cos();
                    let y2 = center_y + inner_radius * angle2.sin();
                    
                    vertices.push(Vertex { position: [center_x, center_y], color });
                    vertices.push(Vertex { position: [x1, y1], color });
                    vertices.push(Vertex { position: [x2, y2], color });
                }
                vertices
            }
            ParticleShape::Diamond => {
                let half_size = size * 0.5;
                vec![
                    Vertex { position: [center_x, center_y - size], color },
                    Vertex { position: [center_x + half_size, center_y], color },
                    Vertex { position: [center_x, center_y + size], color },
                    Vertex { position: [center_x, center_y - size], color },
                    Vertex { position: [center_x, center_y + size], color },
                    Vertex { position: [center_x - half_size, center_y], color },
                ]
            }
        }
    }

    fn generate_vector_vertices(&self, flow_vectors: &[crate::flow_vector::FlowVector]) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let white = [1.0, 1.0, 1.0, 1.0]; // White color with full opacity
        let line_width = 2.0; // Width of the line
        let num_segments = 8; // Number of segments for the rounded caps
        let length_scale = 2.0; // Scale factor for line length

        for vector in flow_vectors {
            let pos = vector.position();
            let dir = vector.direction();
            let mag = dir.length();
            
            // Calculate the end point of the vector with doubled length
            let end_x = pos.x + dir.x * length_scale;
            let end_y = pos.y + dir.y * length_scale;
            
            // Calculate the direction vector and its perpendicular
            let dx = dir.x / mag;
            let dy = dir.y / mag;
            let perp_x = -dy;
            let perp_y = dx;
            
            // Calculate the half-width offset
            let half_width = line_width / 2.0;
            
            // Calculate the four corners of the main rectangle
            let p1 = [pos.x + perp_x * half_width, pos.y + perp_y * half_width];
            let p2 = [pos.x - perp_x * half_width, pos.y - perp_y * half_width];
            let p3 = [end_x - perp_x * half_width, end_y - perp_y * half_width];
            let p4 = [end_x + perp_x * half_width, end_y + perp_y * half_width];
            
            // Add the main rectangle
            vertices.push(Vertex { position: p1, color: white });
            vertices.push(Vertex { position: p2, color: white });
            vertices.push(Vertex { position: p3, color: white });
            
            vertices.push(Vertex { position: p1, color: white });
            vertices.push(Vertex { position: p3, color: white });
            vertices.push(Vertex { position: p4, color: white });
            
            // Add rounded caps at both ends
            for i in 0..num_segments {
                let angle1 = (i as f32 / num_segments as f32) * std::f32::consts::PI;
                let angle2 = ((i + 1) as f32 / num_segments as f32) * std::f32::consts::PI;
                
                // Start cap
                let start_cap_x1 = pos.x + perp_x * half_width * angle1.cos();
                let start_cap_y1 = pos.y + perp_y * half_width * angle1.cos();
                let start_cap_x2 = pos.x + perp_x * half_width * angle2.cos();
                let start_cap_y2 = pos.y + perp_y * half_width * angle2.cos();
                
                vertices.push(Vertex { position: [pos.x, pos.y], color: white });
                vertices.push(Vertex { position: [start_cap_x1, start_cap_y1], color: white });
                vertices.push(Vertex { position: [start_cap_x2, start_cap_y2], color: white });
                
                // End cap
                let end_cap_x1 = end_x + perp_x * half_width * angle1.cos();
                let end_cap_y1 = end_y + perp_y * half_width * angle1.cos();
                let end_cap_x2 = end_x + perp_x * half_width * angle2.cos();
                let end_cap_y2 = end_y + perp_y * half_width * angle2.cos();
                
                vertices.push(Vertex { position: [end_x, end_y], color: white });
                vertices.push(Vertex { position: [end_cap_x1, end_cap_y1], color: white });
                vertices.push(Vertex { position: [end_cap_x2, end_cap_y2], color: white });
            }
        }

        vertices
    }

    pub fn render(
        &mut self,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        background: crate::model::enums::Background,
        flow_vectors: &[crate::flow_vector::FlowVector],
        flow_particles: &[crate::flow_particle::FlowParticle],
        size: (u32, u32),
        should_clear: bool,
    ) {
        // Update uniform buffer with new resolution
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[size.0 as f32, size.1 as f32]));

        // Clear all textures if needed
        if should_clear {
            let clear_color = wgpu::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            };

            // Clear background texture
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Clear Background"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.background_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            // Clear vector texture
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Clear Vector"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.vector_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });

            // Clear particle texture
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Clear Particle"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.particle_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
        }

        // Render background color or vectors
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Background"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.background_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(match background {
                            crate::model::enums::Background::Black => wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            },
                            crate::model::enums::Background::White => wgpu::Color {
                                r: 1.0,
                                g: 1.0,
                                b: 1.0,
                                a: 1.0,
                            },
                            crate::model::enums::Background::Vectors => wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0, // Set alpha to 1.0 for black background
                            },
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // If in vector mode, render the vector field
            if background == crate::model::enums::Background::Vectors {
                // Generate vector vertices with white color
                let vector_vertices = self.generate_vector_vertices(flow_vectors);
                queue.write_buffer(&self.vector_buffer, 0, bytemuck::cast_slice(&vector_vertices));
                self.num_vector_vertices = vector_vertices.len() as u32;

                rpass.set_pipeline(&self.vector_pipeline);
                rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
                rpass.set_vertex_buffer(0, self.vector_buffer.slice(..));
                rpass.draw(0..self.num_vector_vertices, 0..1);
            }
        }

        // Update vertex buffer with particle data
        if !flow_particles.is_empty() {
            let mut vertices = Vec::new();
            for particle in flow_particles {
                let color = [
                    particle.color.r() as f32 / 255.0,
                    particle.color.g() as f32 / 255.0,
                    particle.color.b() as f32 / 255.0,
                    particle.color.a() as f32 / 255.0,
                ];
                
                // Add vertices for the particle shape
                let shape_vertices = self.generate_shape_vertices(
                    particle.xy.x,
                    particle.xy.y,
                    particle.weight() * 2.0, // Scale size based on weight
                    color,
                    particle.shape,
                );
                vertices.extend(shape_vertices);
            }

            // Update vertex buffer with new data
            queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
            self.num_vertices = vertices.len() as u32;
        }

        // Render particles to particle texture
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Particles"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.particle_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rpass.set_pipeline(&self.particle_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            if self.num_vertices > 0 {
                rpass.draw(0..self.num_vertices, 0..1);
            }
        }

        // Composite all layers to the final view
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Composite Layers"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rpass.set_pipeline(&self.quad_pipeline);
            rpass.set_bind_group(0, &self.background_bind_group, &[]);
            rpass.set_bind_group(1, &self.vector_bind_group, &[]);
            rpass.set_bind_group(2, &self.particle_texture_bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }
    }
    
    pub fn resize(&mut self, device: &wgpu::Device, new_size: (u32, u32)) {
        // Recreate layer textures with new size
        self.background_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Background Texture"),
            size: wgpu::Extent3d {
                width: new_size.0,
                height: new_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        self.background_view = self.background_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.vector_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Vector Texture"),
            size: wgpu::Extent3d {
                width: new_size.0,
                height: new_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        self.vector_view = self.vector_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.particle_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Particle Texture"),
            size: wgpu::Extent3d {
                width: new_size.0,
                height: new_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        self.particle_view = self.particle_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Recreate MSAA texture with new size
        self.msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("MSAA Texture"),
            size: wgpu::Extent3d {
                width: new_size.0,
                height: new_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        self.msaa_view = self.msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Recreate resolve texture with new size
        self.resolve_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Resolve Texture"),
            size: wgpu::Extent3d {
                width: new_size.0,
                height: new_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        
        self.resolve_view = self.resolve_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create samplers for each layer
        let background_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Background Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let vector_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Vector Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let particle_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Particle Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Update bind groups with new textures
        self.background_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Background Bind Group"),
            layout: &self.background_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.background_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&background_sampler),
                },
            ],
        });

        self.vector_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Vector Bind Group"),
            layout: &self.vector_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.vector_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&vector_sampler),
                },
            ],
        });

        self.particle_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Bind Group"),
            layout: &self.particle_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.particle_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        self.particle_texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Particle Texture Bind Group"),
            layout: &self.particle_texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.particle_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&particle_sampler),
                },
            ],
        });

        self.size = new_size;
    }
} 