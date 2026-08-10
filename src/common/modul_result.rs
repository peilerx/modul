//! Result type used across generators and ports (static, no `dyn Error`).

use ash::vk;

/// Concrete error type for generators and ports (**A3-STATIC**: no `dyn`).
///
/// Prefer short, actionable strings (`"cubes: renderer cargo missing"`).
pub type ModulResult<T> = Result<T, String>;

/// Map `Result<T, vk::Result>` → [`ModulResult`].
#[inline]
pub fn map_vk<T>(result: Result<T, vk::Result>) -> ModulResult<T> {
    result.map_err(|e| format!("Vulkan error: {e:?}"))
}

/// Map any `Display` error into [`ModulResult`].
#[inline]
pub fn from_err<E: std::fmt::Display, T>(result: Result<T, E>) -> ModulResult<T> {
    result.map_err(|e| e.to_string())
}
