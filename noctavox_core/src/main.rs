fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    noctavox_core::app_core::NoctaVox::new().run();
}
