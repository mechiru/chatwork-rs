use reqwest::multipart::{Form, Part};

use std::borrow::Cow;

crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
        pub(super) room_id: i64,
    }
}

impl Service {
    pub async fn list(&self, list: &List) -> crate::Result<Vec<File>> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/files", room_id = self.room_id; list))
            .await
    }

    pub async fn get(&self, file_id: i64, get: &Get) -> crate::Result<File> {
        self.inner
            .get(crate::path_and_query!("/rooms/{room_id}/files/{file_id}", room_id = self.room_id, file_id = file_id; get))
            .await
    }

    pub async fn create<T>(&self, create: Create<T>) -> crate::Result<FileId>
    where
        T: Into<Cow<'static, str>>,
    {
        let Create { file, message } = create;
        let mut form = Form::new().part("file", file.into());
        if let Some(message) = message {
            form = form.part("message", Part::text(message));
        }
        self.inner
            .multipart(
                crate::path_and_query!("/rooms/{room_id}/files", room_id = self.room_id),
                form,
            )
            .await
    }
}

crate::derive_query_or_form! {
    pub struct List {
        pub account_id: Option<i64>,
    }
}

crate::derive_query_or_form! {
    pub struct Get {
        #[serde(serialize_with = "crate::serde::opt_bool_to_u8")]
        pub create_download_url: Option<bool>,
    }
}

#[derive(Debug)]
pub struct Create<T> {
    file: crate::file::File,
    message: Option<T>,
}

impl Create<&'static str> {
    pub fn new(file: crate::file::File) -> Self {
        Self { file, message: None }
    }

    pub fn message<T>(self, message: T) -> Create<T>
    where
        T: Into<Cow<'static, str>>,
    {
        Create { file: self.file, message: Some(message) }
    }
}

crate::derive_model! {
    pub struct FileId {
        pub file_id: i64,
    }
}

crate::derive_model! {
    pub struct File {
        pub account: crate::v2::Account,
        pub download_url: Option<String>,
        pub file_id: i64,
        pub filename: String,
        pub filesize: i64,
        pub message_id: String,
        pub upload_time: i64,
    }
}
