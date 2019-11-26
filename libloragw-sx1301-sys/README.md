# `libloragw-sx1301-sys`

Low-level bindings to Semtech's
[`libloragw`](https://github.com/Lora-net/lora_gateway/tree/master/libloragw),
a hardware abstraction layer for the
[SX1301](https://www.semtech.com/products/wireless-rf/lora-gateways/sx1301)
concentrator chip.

## Bindings

The generated bindings (`src/bindings.rs`) have to manually generated
if the vendored code (`vendor/libloragw`) changes:

```
bindgen vendor/libloragw/bindgen.h \
    --with-derive-default \
    --whitelist-function "lgw_abort_tx" \
    --whitelist-function "lgw_board_setconf" \
    --whitelist-function "lgw_connect" \
    --whitelist-function "lgw_get_trigcnt" \
    --whitelist-function "lgw_lbt_setconf" \
    --whitelist-function "lgw_receive" \
    --whitelist-function "lgw_rxif_setconf" \
    --whitelist-function "lgw_rxrf_setconf" \
    --whitelist-function "lgw_send" \
    --whitelist-function "lgw_start" \
    --whitelist-function "lgw_status" \
    --whitelist-function "lgw_stop" \
    --whitelist-function "lgw_time_on_air" \
    --whitelist-function "lgw_txgain_setconf" \
    --whitelist-function "lgw_version_info" \
    -o src/bindings.rs
```
