use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Selects a message sender to send messages in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMessageSender {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New message sender for the chat

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    message_sender_id: MessageSender,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatMessageSender {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatMessageSender {}

impl SetChatMessageSender {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatMessageSenderBuilder {
        let mut inner = SetChatMessageSender::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMessageSender".to_string();

        RTDSetChatMessageSenderBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_sender_id(&self) -> &MessageSender {
        &self.message_sender_id
    }
}

#[doc(hidden)]
pub struct RTDSetChatMessageSenderBuilder {
    inner: SetChatMessageSender,
}

impl RTDSetChatMessageSenderBuilder {
    pub fn build(&self) -> SetChatMessageSender {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_sender_id<T: AsRef<MessageSender>>(
        &mut self,
        message_sender_id: T,
    ) -> &mut Self {
        self.inner.message_sender_id = message_sender_id.as_ref().clone();
        self
    }
}

impl AsRef<SetChatMessageSender> for SetChatMessageSender {
    fn as_ref(&self) -> &SetChatMessageSender {
        self
    }
}

impl AsRef<SetChatMessageSender> for RTDSetChatMessageSenderBuilder {
    fn as_ref(&self) -> &SetChatMessageSender {
        &self.inner
    }
}
