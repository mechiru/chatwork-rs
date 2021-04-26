crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub async fn list(&self) -> crate::Result<Vec<Contact>> {
        self.inner.get(crate::path_and_query!("/contacts")).await
    }
}

crate::derive_model! {
    pub struct Contact {
        pub account_id: i64,
        pub avatar_image_url: String,
        pub chatwork_id: String,
        pub department: String,
        pub name: String,
        pub organization_id: i64,
        pub organization_name: String,
        pub room_id: i64,
    }
}
