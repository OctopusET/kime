fn main() {
    print!(
        "{}",
        serde_norway::to_string(&kime_engine_core::RawConfig::default()).unwrap()
    );
}
