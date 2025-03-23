#[macro_export]
macro_rules! errlog {
    ($x: expr) => {
        tracing::error!(
            target = crate::config::targets::MAPPED_ERRORS,
            "({}:{}:{}), {}",
            file!(),
            line!(),
            column!(),
            $x
        );
    };
}

// #[macro_export]
// macro_rules! warnlog {
//     ($x: expr) => {
//         if cfg!(debug_assertions) {
//             tracing::warn!(target = "Warning", "({}:{}:{}), {}", file!(), line!(), column!(), $x);
//         } else {
//             tracing::warn!(target = "Warning", "{}", $x);
//         }
//     };
// }
