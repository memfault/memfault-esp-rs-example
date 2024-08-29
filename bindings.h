#pragma once

// These files are required for the preprocessor to evaluate correctly. Normally
// these are injected into the project via CMAKE, but that is bypassed here for
// the bindings in esp-rs.
#define MEMFAULT_METRICS_USER_HEARTBEAT_DEFS_FILE "memfault_esp_metrics_heartbeat_config.def"
#define MEMFAULT_PLATFORM_CONFIG_FILE "memfault_esp_idf_port_config.h"
#define MEMFAULT_TRACE_REASON_USER_DEFS_FILE "memfault_trace_reason_esp_idf_port_config.def"

// Only required for ESP-IDF v4.4 and earlier.
#define MEMFAULT_FREERTOS_COLLECT_TIMER_STACK_FREE_BYTES 0

#include "external/memfault-firmware-sdk/ports/esp_idf/memfault/include/memfault/esp_port/core.h"
#include "external/memfault-firmware-sdk/ports/esp_idf/memfault/include/memfault/esp_port/http_client.h"
#include "external/memfault-firmware-sdk/components/include/memfault/panics/coredump.h"
#include "external/memfault-firmware-sdk/components/include/memfault/metrics/metrics.h"
