# Memfault esp-rs example

## Environment Setup

Follow the instructions from the [esp-rs book](https://esp-rs.github.io/book/installation/index.html) and
install [`espflash`](https://esp-rs.github.io/book/tooling/espflash.html).

The project is configured for the esp32c3. Other boards are possible, but will require a bit of tweaking.

Set `CONFIG_MEMFAULT_PROJECT_KEY` in `sdkconfig.defaults` to the key of your project.

## Running

Define environment variables `WIFI_SSD` and `WIFI_PASS` for your network.

Building, flashing, and running is as simple as:

```bash
cargo run
```