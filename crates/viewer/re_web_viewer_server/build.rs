fn main() {
    // https://blog.rust-lang.org/2024/05/06/check-cfg.html
    // See `Cargo.toml` docs for info about `__disable_server` and `RERUN_DISABLE_WEB_VIEWER_SERVER`.
    println!("cargo::rustc-check-cfg=cfg(disable_web_viewer_server)");

    let disable_web_viewer_server =
        re_build_tools::is_tracked_env_var_set("RERUN_DISABLE_WEB_VIEWER_SERVER")
            || cfg!(feature = "__disable_server");

    if disable_web_viewer_server {
        println!("cargo::rustc-cfg=disable_web_viewer_server");
    }

    let needs_wasm = !disable_web_viewer_server;
    if needs_wasm {
        let viewer_js_path = std::path::Path::new("./web_viewer/re_viewer.js");
        let viewer_wasm_path = std::path::Path::new("./web_viewer/re_viewer_bg.wasm");

        // Building the web viewer is an optional, toolchain-heavy step.
        // For a plain `cargo build`, gracefully disable the server if the assets are missing.
        if !(viewer_js_path.exists() && viewer_wasm_path.exists()) {
            println!(
                "cargo:warning=Web viewer assets not found; disabling the web viewer server. Run `pixi run rerun-build-web` to enable it."
            );
            println!("cargo::rustc-cfg=disable_web_viewer_server");
        }
    }
}
