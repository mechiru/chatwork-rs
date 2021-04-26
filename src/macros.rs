#[doc(hidden)]
#[macro_export]
macro_rules! __path {
    ($path:expr, $($name:ident=$value:expr,)*) => {
        format!(concat!(crate::__path!($path)), $($name = $value,)*).as_str()
    };
		($($expr:expr),*) => {
        concat!("/v2" $(,$expr)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __query {
    () => {
        Option::<&()>::None
    };
    ($expr:expr) => {
        Some($expr)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! path_and_query {
    ($path:expr $(,$name:ident=$value:expr)+ $(;$query:expr)?) => {
        crate::client::PathAndQuery { path: crate::__path!($path, $($name=$value,)*), query: crate::__query!($($query)*) }
    };
    ($path:expr $(,$query:expr)?) => {
        crate::client::PathAndQuery { path: crate::__path!($path), query: crate::__query!($($query)*) }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! derive_service {
    ($($tt:tt)*) => {
        #[derive(Clone, Debug)]
        $($tt)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! derive_query_or_form {
    ($($tt:tt)*) => {
        #[derive(Clone, PartialEq, Debug, Default, serde::Serialize)]
        $($tt)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! derive_model {
    ($($tt:tt)*) => {
        #[derive(Clone, PartialEq, Debug, serde::Deserialize)]
        $($tt)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! derive_enum {
    ($($tt:tt)*) => {
        #[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        $($tt)*
    };
}
