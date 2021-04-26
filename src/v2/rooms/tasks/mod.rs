mod status;

crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn list(&self, list: &List) -> crate::Result<Vec<Task>> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/tasks", room_id = self.room_id; list))
            .await
    }

    pub async fn get(&self, task_id: i64) -> crate::Result<Task> {
        self.inner
            .get(crate::path_and_query!(
                "/rooms/{room_id}/tasks/{task_id}",
                room_id = self.room_id,
                task_id = task_id
            ))
            .await
    }

    pub async fn create(&self, create: &Create<'_>) -> crate::Result<TaskIds> {
        self.inner
            .post(crate::path_and_query!("/rooms/{room_id}/tasks", room_id = self.room_id), create)
            .await
    }

    pub fn status(&self, task_id: i64) -> status::Service {
        status::Service { inner: self.inner.clone(), room_id: self.room_id, task_id }
    }
}

crate::derive_query_or_form! {
    pub struct List {
        pub account_id: Option<i64>,
        pub assigned_by_account_id: Option<i64>,
        pub status: Option<crate::v2::TaskStatus>,
    }
}

crate::derive_query_or_form! {
    pub struct Create<'a> {
        pub body: &'a str,
        pub limit: Option<i64>,
        pub limit_type: Option<crate::v2::TaskLimit>,
        pub to_ids: crate::v2::List<'a, i64>,
    }
}

crate::derive_model! {
    pub struct Task {
        pub account: crate::v2::Account,
        pub assigned_by_account: crate::v2::Account,
        pub body: String,
        pub limit_time: i64,
        pub limit_type: crate::v2::TaskLimit,
        pub message_id: String,
        pub status: crate::v2::TaskStatus,
        pub task_id: i64,
    }
}

crate::derive_model! {
    pub struct TaskIds {
        pub task_ids: Vec<i64>,
    }
}
