//! Module which contains error types.

use image::ImageError;
use std::io;
use thiserror::Error;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;
use zip::result::ZipError;

pub type SmmdbResult<T> = Result<T, SmmdbError>;
pub(crate) type Course2Result<T> = Result<T, Course2Error>;
#[cfg(all(feature = "save", not(target_arch = "wasm32")))]
pub(crate) type Course2ResultRef<'a, T> = Result<&'a T, &'a Course2Error>;

#[derive(Debug, Error)]
pub enum SmmdbError {
    #[error("Mime type {0} not supported")]
    MimeTypeUnsupported(String),
    /// Failed to decompress zip file
    #[error(transparent)]
    Zip(#[from] ZipError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    CourseError(#[from] CourseError),
    #[error(transparent)]
    Course2Error(#[from] Course2Error),
    #[error(transparent)]
    ImageError(#[from] ImageError),
    #[cfg(feature = "save")]
    #[error(transparent)]
    SaveError(#[from] SaveError),
    #[error(transparent)]
    FromHex(#[from] hex::FromHexError),
}

impl From<SmmdbError> for String {
    fn from(s: SmmdbError) -> String {
        format!("{:?}", s)
    }
}

/// Error which can occur during Super Mario Maker course file serialization.
#[derive(Clone, Debug, Error)]
pub enum CourseError {
    #[error("CourseConvertError::GameStyleParse")]
    GameStyleParse,
    #[error("CourseConvertError::CourseThemeParse")]
    CourseThemeParse,
    #[error("CourseConvertError::AutoScrollParse")]
    AutoScrollParse,
    #[error("CourseConvertError::SoundTypeConvert")]
    SoundTypeConvert,
}

/// Error which can occur during Super Mario Maker 2 course file serialization.
#[derive(Clone, Debug, Error)]
pub enum Course2Error {
    #[error("Course2ConvertError::InvalidDate")]
    InvalidDate {
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
    },
    #[error("Course2ConvertError::GameStyleParse")]
    GameStyleParse,
    #[error("Course2ConvertError::ClearConditionTypeParse")]
    ClearConditionTypeParse,
    #[error("Course2ConvertError::CourseThemeParse")]
    CourseThemeParse,
    #[error("Course2ConvertError::AutoScrollParse")]
    AutoScrollParse,
    #[error("Course2ConvertError::ScreenBoundaryParse")]
    ScreenBoundaryParse,
    #[error("Course2ConvertError::OrientationParse")]
    OrientationParse,
    #[error("Course2ConvertError::WaterModeParse")]
    WaterModeParse,
    #[error("Course2ConvertError::WaterSpeedParse")]
    WaterSpeedParse,
    #[error("Course2ConvertError::DayTimeParse")]
    DayTimeParse,
    #[error("Course2ConvertError::SoundTypeConvert")]
    SoundTypeConvert,
    #[error("Course2ConvertError::ConvertFromBuffer")]
    ConvertFromBuffer,
    #[error("String too long. Expected max length <= 75. Receiced: {0}")]
    StringTooLong(usize),
}

#[cfg(feature = "save")]
#[derive(Clone, Debug, Error)]
pub enum SaveError {
    #[error("index must be between 0 and 180, but received {0}")]
    CourseIndexOutOfBounds(u8),
    #[error("no course found at index {0}")]
    CourseNotFound(u8),
    #[error("thumbnail is missing for course {0}")]
    ThumbnailRequired(String),
    #[error("cannot add corrupted course {0}")]
    CorruptedCourse(Course2Error),
}

#[cfg(target_arch = "wasm32")]
impl From<SmmdbError> for JsValue {
    fn from(err: SmmdbError) -> JsValue {
        JsValue::from(format!("{}", err))
    }
}

#[cfg(target_arch = "wasm32")]
impl From<Course2Error> for JsValue {
    fn from(err: Course2Error) -> JsValue {
        JsValue::from(format!("{}", err))
    }
}
