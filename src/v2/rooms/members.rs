crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn list(&self) -> crate::Result<Vec<Member>> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/members", room_id = self.room_id))
            .await
    }

    pub async fn update<'a, 'b>(&self, update: &'a Update<'b>) -> crate::Result<Members> {
        self.inner
            .put(
                crate::path_and_query!("/rooms/{room_id}/members", room_id = self.room_id),
                Some(update),
            )
            .await
    }
}

crate::derive_query_or_form! {
    pub struct Update<'a> {
        pub members_admin_ids: crate::v2::List<'a, i64>,
        pub members_member_ids: Option<crate::v2::List<'a, i64>>,
        pub members_readonly_ids: Option<crate::v2::List<'a, i64>>,
    }
}

crate::derive_model! {
    pub struct Member {
        pub account_id: i64,
        pub avatar_image_url: String,
        pub chatwork_id: String,
        pub department: String,
        pub name: String,
        pub organization_id: i64,
        pub organization_name: String,
        pub role: crate::v2::rooms::Role,
    }
}

crate::derive_model! {
    pub struct Members {
        pub admin: Vec<i64>,
        pub member: Vec<i64>,
        pub readonly: Vec<i64>,
    }
}
