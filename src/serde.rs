use serde::Serializer;

pub fn bool_to_u8<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u8(if *v { 1 } else { 0 })
}

pub fn opt_bool_to_u8<S>(v: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(v) => bool_to_u8(v, serializer),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bool_to_u8() {
        #[derive(serde::Serialize)]
        struct S {
            #[serde(serialize_with = "bool_to_u8")]
            v: bool,
        }
        assert_eq!(serde_urlencoded::to_string(&S { v: false }).unwrap(), "v=0");
        assert_eq!(serde_urlencoded::to_string(&S { v: true }).unwrap(), "v=1");
    }

    #[test]
    fn test_opt_bool_to_u8() {
        #[derive(serde::Serialize)]
        struct S {
            #[serde(serialize_with = "opt_bool_to_u8")]
            v: Option<bool>,
        }
        assert_eq!(serde_urlencoded::to_string(&S { v: None }).unwrap(), "");
        assert_eq!(serde_urlencoded::to_string(&S { v: Some(false) }).unwrap(), "v=0");
        assert_eq!(serde_urlencoded::to_string(&S { v: Some(true) }).unwrap(), "v=1");
    }
}
