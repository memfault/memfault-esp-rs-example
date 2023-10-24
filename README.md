# Memfault esp-rs example

## Environment Setup

Follow the instructions from the [esp-rs book](https://esp-rs.github.io/book/installation/index.html) and
install [`espflash`](https://esp-rs.github.io/book/tooling/espflash.html).

Additionally this example uses [cargo-make](https://github.com/sagiegurari/cargo-make) for some task
automation. Specifically for adding a [memfault build-id](https://pypi.org/project/mflt-build-id/).

To install cargo make run the following:

```bash
cargo install cargo-make
```

The project is configured for the esp32c3. Other boards are possible, but will require a bit of tweaking.

Set `CONFIG_MEMFAULT_PROJECT_KEY` in `sdkconfig.defaults` to the key of your project.

## Running

Define environment variables `WIFI_SSD` and `WIFI_PASS` for your network.

Building, flashing, and running is as simple as:

```bash
cargo make flash
```

## Integration Breakdown

Memfault is integrated as an
[ESP-IDF Component](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-guides/tools/idf-component-manager.html) via the
[ESP-RS extra components interface](https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md#extra-esp-idf-components).
This integration focuses on coredump/backtrace functionality only. The bindings needed are generated via `bindings.h` header.

See file level comments for more in-depth program flow descriptions.
