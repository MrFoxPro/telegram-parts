use crate::types::{Integer, ParseMode, TextEntities, TextEntity};
use serde::{Deserialize, Serialize};

/// Represents a video to be sent.
#[derive(Clone, Default, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
}

impl InputMediaVideo {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration.
    pub fn with_duration(mut self, value: Integer) -> Self {
        self.duration = Some(value);
        self
    }

    /// Sets a new value for a `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to cover with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }

    /// Sets a new height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.height = Some(value);
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new value for a `supports_streaming` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the uploaded video is suitable for streaming.
    pub fn with_supports_streaming(mut self, value: bool) -> Self {
        self.supports_streaming = Some(value);
        self
    }

    /// Sets a new width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.width = Some(value);
        self
    }
}
