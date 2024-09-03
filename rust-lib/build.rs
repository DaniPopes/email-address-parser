#[cfg(feature = "__generate_tests")]
include!("./build_impl.rs");

fn main() {
    #[cfg(feature = "__generate_tests")]
    real_main();
}
