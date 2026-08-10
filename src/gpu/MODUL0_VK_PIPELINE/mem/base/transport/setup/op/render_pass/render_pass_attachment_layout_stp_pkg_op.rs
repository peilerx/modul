/// Attachment graph preset — setup op (FIX-097). Variants = CAPS (Vulkan Op align).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderPassAttachmentLayoutStpPkgOp {
    /// Enum variant `SIMPLE` — SIMPLE.
    SIMPLE,
    /// Enum variant `MSAA` — MSAA.
    MSAA,
}