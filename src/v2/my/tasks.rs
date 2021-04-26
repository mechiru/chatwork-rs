crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub async fn list(&self, list: &List) -> crate::Result<Vec<Task>> {
        self.inner.get(crate::path_and_query!("/my/tasks", list)).await
    }
}

crate::derive_query_or_form! {
    pub struct List {
        pub assigned_by_account_id: Option<i64>,
        pub status: Option<crate::v2::TaskStatus>,
    }
}

crate::derive_model! {
    pub struct Task {
        pub assigned_by_account: crate::v2::Account,
        pub body: String,
        pub limit_time: i64,
        pub limit_type: crate::v2::TaskLimit,
        pub message_id: String,
        pub room: Room,
        pub status: crate::v2::TaskStatus,
        pub task_id: i64,
    }
}

crate::derive_model! {
    pub struct Room {
        pub icon_path: String,
        pub name: String,
        pub room_id: i64,
    }
}
