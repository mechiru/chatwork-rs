pub mod contacts;
pub mod incoming_requests;
pub mod me;
pub mod my;
pub mod rooms;

crate::derive_service! {
    pub struct Service {
        pub(crate) inner: crate::Client,
    }
}

impl Service {
    pub fn me(&self) -> me::Service {
        me::Service { inner: self.inner.clone() }
    }

    pub fn my(&self) -> my::Service {
        my::Service { inner: self.inner.clone() }
    }

    pub fn contacts(&self) -> contacts::Service {
        contacts::Service { inner: self.inner.clone() }
    }

    pub fn rooms(&self) -> rooms::Service {
        rooms::Service { inner: self.inner.clone() }
    }

    pub fn incoming_requests(&self) -> incoming_requests::Service {
        incoming_requests::Service { inner: self.inner.clone() }
    }
}

crate::derive_enum! {
    pub enum TaskLimit {
        Date,
        None,
        Time,
    }
}

crate::derive_enum! {
    pub enum TaskStatus {
        Done,
        Open,
    }
}

crate::derive_model! {
    pub struct Account {
        pub account_id: i64,
        pub avatar_image_url: String,
        pub name: String,
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct List<'a, T>(pub &'a [T]);

impl<'a, T> serde::Serialize for List<'a, T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut s = String::new();
        let mut i = 0;
        for v in self.0 {
            i += 1;
            s.push_str(&v.to_string());
            if self.0.len() > i {
                s.push(',');
            }
        }
        serializer.serialize_str(&s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_list() {
        #[derive(serde::Serialize)]
        struct S<'a> {
            v: List<'a, i64>,
        }
        assert_eq!(serde_urlencoded::to_string(&S { v: List(&Vec::new()) }).unwrap(), "v=");
        assert_eq!(
            serde_urlencoded::to_string(&S { v: List(&vec![1, 2, 3]) }).unwrap(),
            "v=1%2C2%2C3"
        );
    }
}
