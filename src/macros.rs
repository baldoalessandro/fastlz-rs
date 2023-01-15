/*
 * Give hints to the compiler for branch prediction optimization.
 */
#[macro_export]
macro_rules! fastlz_likely {
    ($x:expr) => (core::intrinsics::likely($x));
}
#[macro_export]
macro_rules! fastlz_unlikely {
    ($x:expr) => (core::intrinsics::unlikely($x));
}

/*
 * Always check for bound when decompressing.
 * Generally it is best to leave it defined.
 */
#[macro_export]
macro_rules! fastlz_bound_check {
    ($x:expr) => {
        if fastlz_unlikely!(!$x) { return 0; }
    };
}
