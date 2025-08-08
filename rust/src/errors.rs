//! Error types for YKS Converter

use std::fmt;

/// Errors that can occur during MML to MIDI conversion
#[derive(Debug, Clone, PartialEq)]
pub enum ConversionError {
    /// MML and instrument count mismatch
    MmlInstCountMismatch {
        mml_count: usize,
        inst_count: usize,
    },
    /// Regex compilation failed
    RegexCompileFailed(String),
    /// MML parsing failed
    MmlParseFailed(String),
    /// Empty track list
    EmptyTrackList,
    /// Event conversion failed
    EventConversionFailed(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::MmlInstCountMismatch { mml_count, inst_count } => {
                write!(f, "MML 갯수({})와 익기 갯수({})가 다릅니다.", mml_count, inst_count)
            }
            ConversionError::RegexCompileFailed(msg) => {
                write!(f, "Regex compile failed: {}", msg)
            }
            ConversionError::MmlParseFailed(msg) => {
                write!(f, "MML parsing failed: {}", msg)
            }
            ConversionError::EmptyTrackList => {
                write!(f, "Track is empty")
            }
            ConversionError::EventConversionFailed(msg) => {
                write!(f, "Event Convert error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ConversionError {}
