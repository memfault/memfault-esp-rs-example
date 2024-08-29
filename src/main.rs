//! This example demonstrates how to use the Memfault SDK with the ESP-IDF-RS project.
//!
//! The example will connect to WiFi and then post a crash to Memfault. The crash will be triggered
//! by writing to a NULL pointer.

use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::memfault::{
    memfault_boot, memfault_esp_port_http_client_post_data, memfault_metrics_heartbeat_debug_print,
    memfault_metrics_heartbeat_debug_trigger, memfault_metrics_heartbeat_set_unsigned,
    MemfaultMetricId, MfltMetricsIndex_kMfltMetricsIndex_test_metric,
};
use heapless::String;
use log::*;

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Setup and connect to WiFi
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs)).unwrap(),
        sys_loop,
    )
    .unwrap();
    connect_wifi(&mut wifi).unwrap();

    unsafe {
        // Configures Memfault. This is required to be called once before any other Memfault functionality
        // is used.
        //
        // See here for implementation:
        // https://github.com/memfault/memfault-firmware-sdk/blob/bfc51680f260d4e47bfdf6fcf378239b25d99726/ports/esp_idf/memfault/common/memfault_platform_core.c#L157
        memfault_boot();

        // Set a metric value. This will be sent to Memfault in the next heartbeat.
        set_metric(MfltMetricsIndex_kMfltMetricsIndex_test_metric as i32);

        memfault_metrics_heartbeat_debug_trigger();
        memfault_metrics_heartbeat_debug_print();

        // Posts data to Memfault via the ESP-IDF HTTP Client. In this case we are just pushing crash
        // data. See below for a more in-depth look at how Memfault transports data.
        // https://docs.memfault.com/docs/mcu/data-from-firmware-to-the-cloud
        memfault_esp_port_http_client_post_data();
    }

    std::thread::sleep(std::time::Duration::from_secs(20));

    // Trigger a fault by writing to a NULL pointer.
    unsafe { std::ptr::null_mut::<i32>().write(42) };

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Sets a metric value to be sent to Memfault in the next heartbeat.
///
/// All metrics IDs are defined via C macros and will be exposed via FFI as
/// MfltMetricsIndex_kMfltMetricsIndex_<metric_name>.
fn set_metric(metric_id: i32) -> i32 {
    unsafe {
        let id = MemfaultMetricId { _impl: metric_id };
        memfault_metrics_heartbeat_set_unsigned(id, 42)
    }
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: String::try_from(SSID).unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: String::try_from(PASSWORD).unwrap(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
