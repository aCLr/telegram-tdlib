use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of message positions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessagePositions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of messages found
    total_count: i32,
    /// List of message positions
    positions: Vec<MessagePosition>,
}

impl RObject for MessagePositions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessagePositions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessagePositionsBuilder {
        let mut inner = MessagePositions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessagePositionsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn positions(&self) -> &Vec<MessagePosition> {
        &self.positions
    }
}

#[doc(hidden)]
pub struct RTDMessagePositionsBuilder {
    inner: MessagePositions,
}

impl RTDMessagePositionsBuilder {
    pub fn build(&self) -> MessagePositions {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn positions(&mut self, positions: Vec<MessagePosition>) -> &mut Self {
        self.inner.positions = positions;
        self
    }
}

impl AsRef<MessagePositions> for MessagePositions {
    fn as_ref(&self) -> &MessagePositions {
        self
    }
}

impl AsRef<MessagePositions> for RTDMessagePositionsBuilder {
    fn as_ref(&self) -> &MessagePositions {
        &self.inner
    }
}
