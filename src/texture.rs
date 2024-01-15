use anyhow::*;
use image::{GenericImage, RgbaImage};

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn create_atlas(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        images: Vec<&[u8]>, // List of image byte slices
        label: Option<&str>,
    ) -> Result<Self, anyhow::Error> {
        // Load images and get their dimensions
        let mut loaded_images = Vec::new();
        let mut total_width = 0;
        let mut max_height = 0;

        for &image_bytes in &images {
            let img = image::load_from_memory(image_bytes)?;
            total_width += img.width();
            if img.height() > max_height {
                max_height = img.height();
            }
            loaded_images.push(img.to_rgba8());
        }

        // Create a new image with the total width and max height
        let mut atlas = RgbaImage::new(total_width, max_height);

        // Copy images into the atlas
        let mut x_offset = 0;
        for img in loaded_images {
            atlas.copy_from(&img, x_offset, 0)?;
            x_offset += img.width();
        }

        // Create texture from atlas
        let dimensions = atlas.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            atlas.as_raw(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Ok(Self {
            texture,
            view,
            sampler,
        })?)
    }
}
