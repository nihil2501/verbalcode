// Maybe can remove last remaining bit of boilerplate by making this a macro.
// Call sites are using `format!` and `await` each time.
pub async fn log(entry: String) {
    #[cfg(target_arch = "wasm32")]
    wasmcloud_interface_logging::log("debug", entry)
        .await
        .iter()
        .next();

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", entry);
}
