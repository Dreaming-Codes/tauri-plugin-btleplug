package com.plugin.btleplug

import android.Manifest
import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Permission
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

@TauriPlugin(
  permissions = [
    Permission(strings = [Manifest.permission.BLUETOOTH], alias = "bluetooth"),
    Permission(strings = [Manifest.permission.BLUETOOTH_ADMIN], alias = "bluetoothAdmin"),
    Permission(strings = [Manifest.permission.BLUETOOTH_ADVERTISE], alias = "bluetoothAdvertise"),
    Permission(strings = [Manifest.permission.BLUETOOTH_CONNECT], alias = "bluetoothConnect"),
    Permission(strings = [Manifest.permission.BLUETOOTH_PRIVILEGED], alias = "bluetoothPrivileged"),
    Permission(strings = [Manifest.permission.BLUETOOTH_SCAN], alias = "bluetoothScan"),
  ]
)
class BtleplugPlugin(private val activity: Activity): Plugin(activity) {
  private external fun init()

  override fun load(webView: WebView) {
    init()
  }
}
