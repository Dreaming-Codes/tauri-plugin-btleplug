package com.plugin.btleplug

import android.Manifest
import android.app.Activity
import android.os.Build
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.BLUETOOTH], alias = "bluetooth"),
    Permission(strings = [Manifest.permission.BLUETOOTH_ADMIN], alias = "bluetoothAdmin"),
    Permission(strings = [Manifest.permission.BLUETOOTH_SCAN], alias = "bluetoothScan"),
    Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT], alias = "bluetoothConnect")
  ]
)
class BtleplugPlugin(private val activity: Activity): Plugin(activity) {
  override fun load(webView: WebView) {
    println("BtleplugPlugin loaded")
    this.trigger("btleplug:ready", JSObject())
  }
}
