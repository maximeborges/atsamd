pub use super::reg::{HeaderDelay, BreakLength, LinCmd};

/// LIN host mode
#[derive(Debug, Clone, Copy)]
pub enum LinHostMode {
    /// Normal USART transmission
    Usart,
    /// Break field is transmitted when DATA is written
    AutoBreak,
    /// Break, sync and identifier fields are transmitted when identifier is written to DATA
    AutoHeader(HeaderDelay, BreakLength),
}

impl From<LinHostMode> for LinCmd {
    fn from(mode: LinHostMode) -> Self {
        match mode {
            LinHostMode::Usart => LinCmd::None,
            LinHostMode::AutoBreak => LinCmd::SoftwareControlTransmitCmd,
            LinHostMode::AutoHeader(_, _) => LinCmd::AutoTransmitCmd,
        }
    }
}
