pub mod status;
pub mod tasks;

crate::derive_service! {
    pub struct Service {
        pub(super) inner: crate::Client,
    }
}

impl Service {
    pub fn status(&self) -> status::Service {
        status::Service { inner: self.inner.clone() }
    }

    pub fn tasks(&self) -> tasks::Service {
        tasks::Service { inner: self.inner.clone() }
    }
}
