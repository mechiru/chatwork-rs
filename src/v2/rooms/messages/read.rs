crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn update(&self, update: &Update<'_>) -> crate::Result<ReadStatus> {
        self.inner
            .put(
                crate::path_and_query!("/rooms/{room_id}/messages/read", room_id = self.room_id),
                Some(update),
            )
            .await
    }
}

crate::derive_query_or_form! {
    pub struct Update<'a> {
        pub message_id: Option<&'a str>,
    }
}

crate::derive_model! {
    pub struct ReadStatus {
        pub mention_num: i64,
        pub unread_num: i64,
    }
}
