# Tauri Plugin btleplug
add to your `proguard-rules.pro` file:
```proguard
#btleplug resources
-keep class com.nonpolynomial.** { *; }
-keep class io.github.gedgygedgy.** { *; }
```

btleplug functions need to be called on the btleplug adhoc runtime:
```rust
#[tauri::command]
async fn your_cool_tauri_command(app_handle: tauri::AppHandle) -> Result<(), ()> {
    app_handle.btleplug().btleplug_context_spawn(async move {
        // your btleplug code here
    }).await.expect("error during btleplug task");
    Ok(())
}
```

### Supported platforms:
- [x] Windows
- [x] Linux
- [ ] MacOS (https://github.com/deviceplug/btleplug/tree/master?tab=readme-ov-file#macos)
- [x] Android
- [ ] iOS (may work, not tested)
- [ ] Web ([not yet supported by btleplug](https://github.com/deviceplug/btleplug/tree/master?tab=readme-ov-file#platform-status))

Basically this plugin only init btleplug for android, other platforms does not require additional setup to use btleplug so if you do not need android support you can just use btleplug directly.
