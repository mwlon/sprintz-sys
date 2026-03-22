fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch != "x86_64" {
        // sprintz requires AVX2 (x86-only); skip compilation on other arches.
        return;
    }

    let vendor = "vendor/sprintz/cpp/Compress";

    cc::Build::new()
        .cpp(true)
        .flag("-march=haswell") // enables AVX2 + BMI2 + LZCNT (all required by sprintz)
        .flag("-std=c++14")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-variable")
        .include(vendor)
        .files([
            "src/wrapper.cpp",
            &format!("{vendor}/sprintz.cpp"),
            &format!("{vendor}/sprintz_delta.cpp"),
            &format!("{vendor}/sprintz_delta_lowdim.cpp"),
            &format!("{vendor}/sprintz_delta_rle.cpp"),
            &format!("{vendor}/sprintz_xff.cpp"),
            &format!("{vendor}/sprintz_xff_lowdim.cpp"),
            &format!("{vendor}/sprintz_xff_rle.cpp"),
            &format!("{vendor}/delta.cpp"),
            &format!("{vendor}/predict.cpp"),
            &format!("{vendor}/format.cpp"),
        ])
        .compile("sprintz");
}
