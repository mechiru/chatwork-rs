crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub async fn list(&self) -> crate::Result<Vec<IncomingRequest>> {
        self.inner.get(crate::path_and_query!("/incoming_requests")).await
    }

    pub async fn update(&self, request_id: i64) -> crate::Result<IncomingRequest> {
        self.inner
            .put(
                crate::path_and_query!("/incoming_requests/{request_id}", request_id = request_id),
                Option::<&()>::None,
            )
            .await
    }

    pub async fn delete(&self, request_id: i64) -> crate::Result<()> {
        self.inner
            .delete(
                crate::path_and_query!("/incoming_requests/{request_id}", request_id = request_id),
                Option::<&()>::None,
            )
            .await
    }
}

crate::derive_model! {
    pub struct IncomingRequest {
        pub account_id: i64,
        pub avatar_image_url: String,
        pub chatwork_id: String,
        pub department: String,
        pub message: Option<String>,
        pub name: String,
        pub organization_id: i64,
        pub organization_name: String,
        pub request_id: i64,
    }
}
