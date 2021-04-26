crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
        pub(super) task_id: i64,
    }
}

impl Service {
    pub async fn update(&self, update: &Update) -> crate::Result<TaskId> {
        self.inner
            .put(
                crate::path_and_query!(
                    "/rooms/{room_id}/tasks/{task_id}/status",
                    room_id = self.room_id,
                    task_id = self.task_id
                ),
                Some(update),
            )
            .await
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize)]
pub struct Update {
    pub body: crate::v2::TaskStatus,
}

crate::derive_model! {
    pub struct TaskId {
        pub task_id: i64,
    }
}
