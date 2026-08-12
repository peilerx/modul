//! Session-level intents.

pub mod sample_count_prefer_prt;
pub mod tandem_session_prt;
pub mod validation_prefer_prt;

pub use sample_count_prefer_prt::SampleCountPreferPrt;
pub use tandem_session_prt::TandemSessionPrt;
pub use validation_prefer_prt::ValidationPreferPrt;
