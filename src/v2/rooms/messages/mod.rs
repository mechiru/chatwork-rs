mod read;
mod unread;

crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn list(&self, list: &List) -> crate::Result<Vec<Message>> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/messages", room_id = self.room_id; list))
            .await
    }

    pub async fn get(&self, message_id: impl AsRef<str>) -> crate::Result<Message> {
        self.inner
            .get(crate::path_and_query!(
                "/rooms/{room_id}/messages/{message_id}",
                room_id = self.room_id,
                message_id = message_id.as_ref()
            ))
            .await
    }

    pub async fn create(&self, create: &Create<'_>) -> crate::Result<MessageId> {
        self.inner
            .post(
                crate::path_and_query!("/rooms/{room_id}/messages", room_id = self.room_id),
                create,
            )
            .await
    }

    pub async fn update(
        &self,
        message_id: impl AsRef<str>,
        update: &Update<'_>,
    ) -> crate::Result<MessageId> {
        self.inner
            .put(
                crate::path_and_query!(
                    "/rooms/{room_id}/messages/{message_id}",
                    room_id = self.room_id,
                    message_id = message_id.as_ref()
                ),
                Some(update),
            )
            .await
    }

    pub async fn delete(&self, message_id: impl AsRef<str>) -> crate::Result<MessageId> {
        self.inner
            .delete(
                crate::path_and_query!(
                    "/rooms/{room_id}/messages/{message_id}",
                    room_id = self.room_id,
                    message_id = message_id.as_ref()
                ),
                Option::<&()>::None,
            )
            .await
    }

    pub fn read(&self) -> read::Service {
        read::Service { inner: self.inner.clone(), room_id: self.room_id }
    }

    pub fn unread(&self) -> unread::Service {
        unread::Service { inner: self.inner.clone(), room_id: self.room_id }
    }
}

crate::derive_query_or_form! {
    pub struct List {
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub force: Option<bool>,
    }
}

crate::derive_query_or_form! {
    pub struct Create<'a> {
        pub body: &'a str,
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub self_unread: Option<bool>,
    }
}

crate::derive_query_or_form! {
    pub struct Update<'a> {
        pub body: &'a str,
    }
}

crate::derive_model! {
    pub struct MessageId {
        pub message_id: String,
    }
}

crate::derive_model! {
    pub struct Message {
        pub account: crate::v2::Account,
        pub body: String,
        pub message_id: String,
        pub send_time: i64,
        pub update_time: i64,
    }
}
