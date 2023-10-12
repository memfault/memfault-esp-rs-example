# Memfault esp-rs example

## Environment Setup

Follow the instructions from the [esp-rs book](https://esp-rs.github.io/book/installation/index.html) and
install [`espflash`](https://esp-rs.github.io/book/tooling/espflash.html).

The project is configured for the esp32c3. Other boards are possible, but will require a bit of tweaking.

Set `CONFIG_MEMFAULT_PROJECT_KEY` in `sdkconfig.defaults` to the key of your project.

### ESP Component patch

A small patch is needed until this dependency change is upstreamed in the public SDK.

```git
diff --git a/ports/esp_idf/memfault/CMakeLists.txt b/ports/esp_idf/memfault/CMakeLists.txt
index d52cac4..f9dba56 100644
--- a/ports/esp_idf/memfault/CMakeLists.txt
+++ b/ports/esp_idf/memfault/CMakeLists.txt
@@ -160,7 +160,7 @@ if(CONFIG_MEMFAULT_MBEDTLS_METRICS)
 endif()

 if($ENV{MEMFAULT_ESP_HTTP_CLIENT_ENABLE})
-  list(APPEND COMPONENT_REQUIRES esp_http_client)
+  list(APPEND COMPONENT_REQUIRES esp_http_client esp_https_ota)
 endif()
 register_component()
```

## Running

Define environment variables `WIFI_SSD` and `WIFI_PASS` for your network.

Building, flashing, and running is as simple as:

```bash
cargo run
```