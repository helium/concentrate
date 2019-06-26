# `libloragw-sys`

Low-level bindings to Semtech's
[`libloragw`](https://github.com/Lora-net/lora_gateway/tree/master/libloragw),
a hardware abstraction layer for the
[SX1301](https://www.semtech.com/products/wireless-rf/lora-gateways/sx1301)
concentrator chip.

## Bindings

The generated bindings (`src/bindings.rs`) have to manually generated
if the vendored code (`vendor/libloragw`) changes:

```
bindgen vendor/libloragw/wrap.h \
    --with-derive-default \
    --whitelist-function "lgw_board_setconf" \
    --whitelist-function "lgw_lbt_setconf" \
    --whitelist-function "lgw_rxrf_setconf" \
    --whitelist-function "lgw_rxif_setconf" \
    --whitelist-function "lgw_txgain_setconf" \
    --whitelist-function "lgw_start" \
    --whitelist-function "lgw_stop" \
    --whitelist-function "lgw_receive" \
    --whitelist-function "lgw_send" \
    --whitelist-function "lgw_status" \
    --whitelist-function "lgw_abort_tx" \
    --whitelist-function "lgw_get_trigcnt" \
    --whitelist-function "lgw_version_info" \
    --whitelist-function "lgw_time_on_air" \
    --whitelist-function "lgw_gps_enable" \
    --whitelist-function "lgw_gps_disable" \
    --whitelist-function "lgw_parse_nmea" \
    --whitelist-function "lgw_parse_ubx" \
    --whitelist-function "lgw_gps_get" \
    --whitelist-function "lgw_gps_sync" \
    --whitelist-function "lgw_cnt2utc" \
    --whitelist-function "lgw_utc2cnt" \
    --whitelist-function "lgw_cnt2gps" \
    --whitelist-function "lgw_gps2cnt" \
    -o src/bindings.rs
```
