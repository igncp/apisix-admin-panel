macro_rules! derive_common {
    ($i:item) => {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        $i
    };
}

macro_rules! derive_common_default {
    ($i:item) => {
        crate::macros::derive_common! {
        #[derive(Default)]
        $i
        }
    };
}

pub(crate) use derive_common;
pub(crate) use derive_common_default;
