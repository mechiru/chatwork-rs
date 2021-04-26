crate::derive_service!(
    pub struct Service {
        pub(super) inner: crate::Client,
    }
);

impl Service {
    pub async fn get(&self) -> crate::Result<Me> {
        self.inner.get(crate::path_and_query!("/me")).await
    }
}

crate::derive_model! {
    pub struct Me {
        pub account_id: i64,
        pub avatar_image_url: String,
        pub chatwork_id: String,
        pub department: String,
        pub facebook: String,
        pub introduction: String,
        pub login_mail: String,
        pub mail: String,
        pub name: String,
        pub organization_id: i64,
        pub organization_name: String,
        pub room_id: i64,
        pub skype: String,
        pub tel_extension: String,
        pub tel_mobile: String,
        pub tel_organization: String,
        pub title: String,
        pub twitter: String,
        pub url: String,
    }
}
