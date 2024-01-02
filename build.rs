fn main() -> miette::Result<()> {
    let mut b = autocxx_build::Builder::new(
        "src/main.rs",
        &[
            "src",
            "hlsdk/cl_dll",
            "hlsdk/common",
            "hlsdk/dlls",
            "hlsdk/engine",
            "hlsdk/game_shared",
            "hlsdk/pm_shared",
            "hlsdk/public",
        ],
    )
    .build()?;
    // This assumes all your C++ bindings are in main.rs
    b.compile("autocxx-bug-operator-new"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
