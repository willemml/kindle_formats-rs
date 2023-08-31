//! Rust structure representation of reader data files (.yjr and
//! .azw3r) and timer data files (.yjf and .azw3f).
//!
//! These representations may be innacurate. They may be missing some
//! fields and some fields may not be required by the format even
//! though I have not wrapped them in optionals. These representations
//! may seem strange, but they are designed to go with my other
//! project [serde_krds](https://github.com/willemml/serde_krds).
//!
//! Information on format and value/field names were determined thanks to [this mobilereads.com thread](https://www.mobileread.com/forums/showthread.php?t=322172)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Timer data file (.yjf and .azw3f) contains reading statistics such
/// as words per minute and percent of book read.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct TimerDataFile {
    #[serde(rename = "timer.model", skip_serializing_if = "Option::is_none")]
    pub timer_model: Option<TimerModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpr: Option<FPR>,
    #[serde(rename = "book.info.store", skip_serializing_if = "Option::is_none")]
    pub book_info_store: Option<BookInfoStore>,
    #[serde(rename = "page.history.store", skip_serializing_if = "Option::is_none")]
    pub page_history_store: Option<Vec<PHRWrapper>>,
    #[serde(
        rename = "whisperstore.migration.status",
        skip_serializing_if = "Option::is_none"
    )]
    pub whisperstore_migration_status: Option<WhisperstoreMigrationStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lpr: Option<LPR>,
}

/// Reader data file (.yjr and .azw3r) contains the users current font
/// configuration (including size, bold, chosen font, etc...) as well
/// as a list of different annotations the user has made (handwritten
/// (Scribe), sticky notes (PDFs), typed notes, highlights and
/// bookmarks).
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct ReaderDataFile {
    #[serde(rename = "font.prefs", skip_serializing_if = "Option::is_none")]
    pub font_preferences: Option<FontPreferences>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_lpr: Option<bool>,
    #[serde(
        rename = "next.in.series.info.data",
        skip_serializing_if = "Option::is_none"
    )]
    pub nis_info_data: Option<String>,
    #[serde(
        rename = "annotation.cache.object",
        skip_serializing_if = "Option::is_none"
    )]
    pub annotation_cache: Option<HashMap<NoteType, IntervalTree<Note>>>,
    #[serde(rename = "apnx.key", skip_serializing_if = "Option::is_none")]
    pub apnx_key: Option<APNXKey>,
    #[serde(rename = "language.store", skip_serializing_if = "Option::is_none")]
    pub language_store: Option<LanguageStore>,
    #[serde(rename = "ReaderMetrics", skip_serializing_if = "Option::is_none")]
    pub reader_metrics: Option<HashMap<String, String>>,
}

/// The purpose of this data type is unknown to me, if you know what
/// it is, please let me know.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct FPR(pub String, pub i64, pub i64, pub String, pub String);

/// The purpose of this data type is unknown to me, if you know what
/// it is, please let me know.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct LPR(pub i8, pub String, pub i64);

/// The purpose of the fields of this data type is unknown to me, if
/// you know what it is, please let me know.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct WhisperstoreMigrationStatus(pub bool, pub bool);

/// Stores the calculator data for timer data files, this data is used
/// to calculate reading speed and estimated time left.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct TimerModel(
    /// Version
    pub i64,
    /// Total time reading (ms)
    pub i64,
    /// Total words read
    pub i64,
    /// Total percent of book read
    pub f64,
    /// Calculator data
    pub TACWrapper,
);

/// Simple information on how much the user has read.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct BookInfoStore(
    /// Number of words
    pub i64,
    /// Percent read
    pub f64,
);

/// Wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
#[serde(rename = "page.history.record")]
pub struct PHRWrapper(pub PageHistoryRecord);

/// A record of when a user read or started reading a page (unsure.)
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct PageHistoryRecord(
    /// Position
    pub String,
    /// Time
    pub i64,
);

/// Wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
#[serde(rename = "timer.average.calculator")]
pub struct TACWrapper(pub TimerAverageCalculator);

/// Statistical distribution data storage for the timer model.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct TimerAverageCalculator(
    pub i32,
    pub i32,
    /// Normal distributions
    pub Vec<TADNWrapper>,
    /// Outliers
    pub Vec<TAOWrapper>,
);

/// Wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
#[serde(rename = "timer.average.calculator.distribution.normal")]
pub struct TADNWrapper(pub TimerAverageDistributionNormal);

/// Normal distribution for reading speed data.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct TimerAverageDistributionNormal(
    /// Sample count
    pub i64,
    /// Sum of samples (words per minute)
    pub f64,
    pub f64,
);

/// Wrapper type.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
#[serde(rename = "timer.average.calculator.outliers")]
pub struct TAOWrapper(pub TimerAverageOutliers);

/// Outliers for reading speed data.
#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq)]
pub struct TimerAverageOutliers(pub i32, pub f64, pub f64);

/// Font preferences used while reading document. I do not know the
/// purpose of some of the fields in this data type, if you know them,
/// please let me know.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FontPreferences(
    /// Selected font
    pub String,
    pub i32,
    /// Font size
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    /// Bold level
    pub i32,
    pub String,
    pub i32,
    pub String,
    pub bool,
    pub String,
    pub i32,
);

/// The purpose of this data type is unknown to me, if you know what
/// it is, please let me know.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct APNXKey(
    pub String,
    pub String, // type
    pub bool,
    pub Vec<i32>,
    pub i32,
    pub i32,
    pub i32,
    pub String,
);

/// Document annotation data (bookmarks, highlights, and notes.)
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct AnnotationData(
    /// Start position
    pub String,
    /// End position
    pub String,
    /// Date created
    pub i64,
    /// Date last modified
    pub i64,
    /// Template (always "0x30 0xef 0xbf 0xbc 0x30")
    pub String,
    /// NBK reference of note (if drawing, e.g. on Scribe) otherwise
    /// plaintext of note contents. Not present in highlights.
    pub Option<String>,
);

/// Determines the type of each note, used as the field in the
/// annotation data map. Encoded as a 32 bit integer.
#[repr(i32)]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum NoteType {
    Bookmark = 0,
    Highlight = 1,
    Typed = 2,
    Handwritten = 10,
    Sticky = 11,
}

impl TryFrom<i32> for NoteType {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        Ok(match value {
            0 => Self::Bookmark,
            1 => Self::Highlight,
            2 => Self::Typed,
            10 => Self::Handwritten,
            11 => Self::Sticky,
            _ => return Err(()),
        })
    }
}

impl Serialize for NoteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

use serde::de::{self, Visitor};

struct NoteTypeVisitor;

impl<'de> Visitor<'de> for NoteTypeVisitor {
    type Value = NoteType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .try_into()
            .map_err(|_| E::custom(format!("i32 out of range: -2..9")))
    }
}

impl<'de> Deserialize<'de> for NoteType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i32(NoteTypeVisitor)
    }
}

/// Wrapper type used for storing annotation data in reader data files.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename = "saved.avl.interval.tree")]
pub struct IntervalTree<T>(pub Vec<T>);

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum Note {
    #[serde(rename = "annotation.personal.bookmark")]
    Bookmark(AnnotationData),
    #[serde(rename = "annotation.personal.highlight")]
    Highlight(AnnotationData),
    #[serde(rename = "annotation.personal.note")]
    Typed(AnnotationData),
    #[serde(rename = "annotation.personal.handwritten_note")]
    Handwritten(AnnotationData),
    #[serde(rename = "annotation.personal.sticky_note")]
    Sticky(AnnotationData),
}

/// Appears to store the system language of the Kindle that created
/// the file. The purpose of the integer is unknown to me.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LanguageStore(pub String, pub i32);

/// Tells whether or not the book was launched before on the Kindle
/// that created the reader data file.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReaderMetrics {
    pub booklaunchedbefore: String,
}
