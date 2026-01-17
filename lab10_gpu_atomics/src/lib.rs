use wgpu::util::DeviceExt;
use std::borrow::Cow;

pub async fn run_atomic_frontier(num_threads: u32) -> (u32, Vec<u32>) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let counter_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Counter"),
        contents: bytemuck::cast_slice(&[0u32]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    // Make frontier large enough
    let frontier_size = (num_threads as usize * std::mem::size_of::<u32>()) as wgpu::BufferAddress;
    let frontier_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Frontier"),
        size: frontier_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: counter_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: frontier_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(num_threads, 1, 1);
    }

    // Copy back buffers
    let counter_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Counter Staging"),
        size: 4,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let frontier_staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Frontier Staging"),
        size: frontier_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&counter_buffer, 0, &counter_staging, 0, 4);
    encoder.copy_buffer_to_buffer(&frontier_buffer, 0, &frontier_staging, 0, frontier_size);

    queue.submit(Some(encoder.finish()));

    let slice_counter = counter_staging.slice(..);
    let slice_frontier = frontier_staging.slice(..);

    let (tx, rx) = std::sync::mpsc::channel();
    slice_counter.map_async(wgpu::MapMode::Read, move |res| tx.send(res).unwrap());

    let (tx2, rx2) = std::sync::mpsc::channel();
    slice_frontier.map_async(wgpu::MapMode::Read, move |res| tx2.send(res).unwrap());

    device.poll(wgpu::Maintain::Wait);
    rx.recv().unwrap().unwrap();
    rx2.recv().unwrap().unwrap();

    let counter_data = slice_counter.get_mapped_range();
    let final_count = *bytemuck::from_bytes::<u32>(&counter_data);
    drop(counter_data);

    let frontier_data = slice_frontier.get_mapped_range();
    let frontier = bytemuck::cast_slice(&frontier_data).to_vec();
    drop(frontier_data);

    counter_staging.unmap();
    frontier_staging.unmap();

    (final_count, frontier)
}
