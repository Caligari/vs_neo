pub type WindowExtent = u16;
pub type WindowDepth = u16;
pub type WindowBuffers = u8;

#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    Window,
    Fullscreen,
    FullscreenWindow,
}
