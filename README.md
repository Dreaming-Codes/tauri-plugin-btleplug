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
