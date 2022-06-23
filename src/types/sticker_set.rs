use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the sticker set

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
    /// Title of the sticker set
    title: String,
    /// Name of the sticker set
    name: String,
    /// Sticker set thumbnail in WEBP, TGS, or WEBM format with width and height 100; may be null. The file can be downloaded only before the thumbnail is changed
    thumbnail: Option<Thumbnail>,
    /// Sticker set thumbnail's outline represented as a list of closed vector paths; may be empty. The coordinate system origin is in the upper-left corner
    thumbnail_outline: Vec<ClosedVectorPath>,
    /// True, if the sticker set has been installed by the current user
    is_installed: bool,
    /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously
    is_archived: bool,
    /// True, if the sticker set is official
    is_official: bool,
    /// Type of the stickers in the set

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// True for already viewed trending sticker sets
    is_viewed: bool,
    /// List of stickers in this set
    stickers: Vec<Sticker>,
    /// A list of emoji corresponding to the stickers in the same order. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    emojis: Vec<Emojis>,
}

impl RObject for StickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStickerSetBuilder {
        let mut inner = StickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStickerSetBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn thumbnail_outline(&self) -> &Vec<ClosedVectorPath> {
        &self.thumbnail_outline
    }

    pub fn is_installed(&self) -> bool {
        self.is_installed
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn is_official(&self) -> bool {
        self.is_official
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn is_viewed(&self) -> bool {
        self.is_viewed
    }

    pub fn stickers(&self) -> &Vec<Sticker> {
        &self.stickers
    }

    pub fn emojis(&self) -> &Vec<Emojis> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct RTDStickerSetBuilder {
    inner: StickerSet,
}

impl RTDStickerSetBuilder {
    pub fn build(&self) -> StickerSet {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn thumbnail_outline(&mut self, thumbnail_outline: Vec<ClosedVectorPath>) -> &mut Self {
        self.inner.thumbnail_outline = thumbnail_outline;
        self
    }

    pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
        self.inner.is_installed = is_installed;
        self
    }

    pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
        self.inner.is_archived = is_archived;
        self
    }

    pub fn is_official(&mut self, is_official: bool) -> &mut Self {
        self.inner.is_official = is_official;
        self
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }

    pub fn is_viewed(&mut self, is_viewed: bool) -> &mut Self {
        self.inner.is_viewed = is_viewed;
        self
    }

    pub fn stickers(&mut self, stickers: Vec<Sticker>) -> &mut Self {
        self.inner.stickers = stickers;
        self
    }

    pub fn emojis(&mut self, emojis: Vec<Emojis>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }
}

impl AsRef<StickerSet> for StickerSet {
    fn as_ref(&self) -> &StickerSet {
        self
    }
}

impl AsRef<StickerSet> for RTDStickerSetBuilder {
    fn as_ref(&self) -> &StickerSet {
        &self.inner
    }
}
