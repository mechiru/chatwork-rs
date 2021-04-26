pub mod files;
mod link;
pub mod members;
pub mod messages;
mod tasks;

crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub async fn list(&self) -> crate::Result<Vec<Room>> {
        self.inner.get(crate::path_and_query!("/rooms")).await
    }

    pub async fn get(&self, room_id: i64) -> crate::Result<Room> {
        self.inner.get(crate::path_and_query!("/rooms/{room_id}", room_id = room_id)).await
    }

    pub async fn create(&self, create: &Create<'_>) -> crate::Result<RoomId> {
        self.inner.post(crate::path_and_query!("/rooms"), create).await
    }

    pub async fn update(&self, room_id: i64, update: &Update<'_>) -> crate::Result<RoomId> {
        self.inner
            .put(crate::path_and_query!("/rooms/{room_id}", room_id = room_id), Some(update))
            .await
    }

    pub async fn delete(&self, room_id: i64, delete: &Delete) -> crate::Result<()> {
        self.inner
            .delete(crate::path_and_query!("/rooms/{room_id}", room_id = room_id), Some(delete))
            .await
    }

    pub fn members(&self, room_id: i64) -> members::Service {
        members::Service { inner: self.inner.clone(), room_id }
    }

    pub fn messages(&self, room_id: i64) -> messages::Service {
        messages::Service { inner: self.inner.clone(), room_id }
    }

    pub fn tasks(&self, room_id: i64) -> tasks::Service {
        tasks::Service { inner: self.inner.clone(), room_id }
    }

    pub fn files(&self, room_id: i64) -> files::Service {
        files::Service { inner: self.inner.clone(), room_id }
    }

    pub fn link(&self, room_id: i64) -> link::Service {
        link::Service { inner: self.inner.clone(), room_id }
    }
}

crate::derive_enum! {
    pub enum Role {
        Admin,
        Member,
        Readonly,
    }
}

crate::derive_enum! {
    pub enum Type {
        Direct,
        Group,
        My,
    }
}

crate::derive_enum! {
    pub enum Icon {
        Group,
        Check,
        Document,
        Meeting,
        Event,
        Project,
        Business,
        Study,
        Security,
        Star,
        Idea,
        Heart,
        Magcup,
        Beer,
        Music,
        Sports,
        Travel,
    }
}

crate::derive_enum! {
    pub enum Action {
        Leave,
        Delete,
    }
}

crate::derive_query_or_form! {
    pub struct Update<'a> {
        pub description: Option<&'a str>,
        pub icon_preset: Option<Icon>,
        pub name: Option<&'a str>,
    }
}

crate::derive_query_or_form! {
    pub struct Create<'a> {
        pub description: Option<&'a str>,
        pub icon_preset: Option<Icon>,
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub link: Option<bool>,
        pub link_code: Option<&'a str>,
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub link_need_acceptance: Option<bool>,
        pub members_admin_ids: crate::v2::List<'a, i64>,
        pub members_member_ids: Option<crate::v2::List<'a, i64>>,
        pub members_readonly_ids: Option<crate::v2::List<'a, i64>>,
        pub name: &'a str,
    }
}

#[derive(Clone, PartialEq, Debug, serde::Serialize)]
pub struct Delete {
    pub action_type: Action,
}

crate::derive_model! {
    pub struct RoomId {
        pub room_id: i64,
    }
}

crate::derive_model! {
    pub struct Room {
        pub file_num: i64,
        pub icon_path: String,
        pub last_update_time: i64,
        pub mention_num: i64,
        pub message_num: i64,
        pub mytask_num: i64,
        pub name: String,
        pub role: Role,
        pub room_id: i64,
        pub sticky: bool,
        pub task_num: i64,
        pub r#type: Type,
        pub unread_num: i64,
    }
}
