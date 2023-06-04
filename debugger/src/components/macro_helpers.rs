#[macro_export]
macro_rules! bool {
    ( $f:literal, $v:expr) => {
        format!($f, if $v { "⬛" } else { "⬜" })
    };
}
