use reqwest::{multipart::Part, Body};

use std::borrow::Cow;

#[derive(Debug)]
pub struct File {
    part: Part,
}

impl File {
    pub fn text<N, C>(name: N, content: C) -> crate::Result<Self>
    where
        N: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>,
    {
        let part = Part::text(content).file_name(name).mime_str("text/plain")?;
        Ok(Self { part })
    }

    pub fn bytes<N, C>(name: N, content: C, mime: impl AsRef<str>) -> crate::Result<Self>
    where
        N: Into<Cow<'static, str>>,
        C: Into<Cow<'static, [u8]>>,
    {
        let part = Part::bytes(content).file_name(name).mime_str(mime.as_ref())?;
        Ok(Self { part })
    }

    pub fn stream<N, C>(name: N, content: C, mime: impl AsRef<str>) -> crate::Result<Self>
    where
        N: Into<Cow<'static, str>>,
        C: Into<Body>,
    {
        let part = Part::stream(content).file_name(name).mime_str(mime.as_ref())?;
        Ok(Self { part })
    }
}

impl From<File> for Part {
    fn from(file: File) -> Self {
        file.part
    }
}
