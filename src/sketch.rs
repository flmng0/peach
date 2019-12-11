use crate::Size;

use winit::window::Window;

#[derive(Debug)]
pub struct Sketch {
    device: wgpu::Device,
    queue: wgpu::Queue,

    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline: wgpu::RenderPipeline,
}

impl Sketch {
    pub(crate) fn new(_window: &Window) -> Self {
        unimplemented!();
    }

    pub(crate) fn resize(&mut self, _size: Size) {}

    pub(crate) fn finish(&mut self) {}
}
