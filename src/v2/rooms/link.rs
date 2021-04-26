crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn get(&self) -> crate::Result<Link> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/link", room_id = self.room_id))
            .await
    }

    pub async fn create(&self, create: &Create<'_>) -> crate::Result<Link> {
        self.inner
            .post(crate::path_and_query!("/rooms/{room_id}/link", room_id = self.room_id), create)
            .await
    }

    pub async fn update(&self, update: &Update<'_>) -> crate::Result<Link> {
        self.inner
            .put(
                crate::path_and_query!("/rooms/{room_id}/link", room_id = self.room_id),
                Some(update),
            )
            .await
    }

    pub async fn delete(&self) -> crate::Result<Visibility> {
        self.inner
            .delete(
                crate::path_and_query!("/rooms/{room_id}/link", room_id = self.room_id),
                Option::<&()>::None,
            )
            .await
    }
}

crate::derive_query_or_form! {
    pub struct Create<'a> {
        pub code: Option<&'a str>,
        pub description: Option<&'a str>,
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub need_acceptance: Option<bool>,
    }
}

crate::derive_query_or_form! {
    pub struct Update<'a> {
        pub code: Option<&'a str>,
        pub description: Option<&'a str>,
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub need_acceptance: Option<bool>,
    }
}

crate::derive_model! {
    pub struct Link {
        pub description: Option<String>,
        pub need_acceptance: Option<bool>,
        pub public: bool,
        pub url: Option<String>,
    }
}

crate::derive_model! {
    pub struct Visibility {
        pub public: bool,
    }
}
