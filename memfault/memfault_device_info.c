#include "memfault/core/platform/device_info.h"

void memfault_platform_get_device_info(sMemfaultDeviceInfo *info)
{
    // platform specific version information
    *info = (sMemfaultDeviceInfo){
        .device_serial = "esp-rust",
        .software_type = "main",
        .software_version = "1.0.0",
        .hardware_version = "evt",
    };
}