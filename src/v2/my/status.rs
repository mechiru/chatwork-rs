crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub async fn get(&self) -> crate::Result<Status> {
        self.inner.get(crate::path_and_query!("/my/status")).await
    }
}

crate::derive_model! {
    pub struct Status {
        pub mention_num: i64,
        pub mention_room_num: i64,
        pub mytask_num: i64,
        pub mytask_room_num: i64,
        pub unread_num: i64,
        pub unread_room_num: i64,
    }
}
