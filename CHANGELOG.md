<!--
M.m.p (YYYY-MM-DD)
==================
Add a summary of this release.

**BREAKING CHANGES**:

* Some change which breaks API or ABI compatiblity with.


Feature enhancements:

* [Link to github PR]():
  A new feature.

Bug fixes:

* [Link to github PR]():
  A bugfix.
-->

0.2.9 (2019-11-27)
==================
Feature enhancements:

* [#48](https://github.com/helium/concentrate/pull/48):
  Allow runtime choice of logging backend.

0.2.8 (2019-11-26)
==================
Housekeeping:

* [#47](https://github.com/helium/concentrate/pull/47):
  Add SX1302 support.

0.2.7 (2019-11-18)
==================
Feature enhancements:

* [#46](https://github.com/helium/concentrate/pull/46):
  Add connect `subcommand`.

0.2.6 (2019-11-08)
==================
Feature enhancements:

* [#45](https://github.com/helium/concentrate/pull/45):
  Add tag bits to proto message.

0.2.4 (2019-10-01)
==================
Bug fixes:

* [#36](https://github.com/helium/concentrate/pull/36):
  Take timestamp, snr, rssi from first fragment.

Feature enhancements:

* [#37](https://github.com/helium/concentrate/pull/37):
  Make `concentrate` executable the root level project.

* [#38](https://github.com/helium/concentrate/pull/38):
  Upgrade to the 2018 edition.

0.2.3 (2019-08-08)
==================
Bug fixes:

* [#34](https://github.com/helium/concentrate/pull/34):
  Default poll interval changed from 100ms to 10ms.
  Epoll triggering mode set to level instead of edge.

0.2.2 (2019-08-08)
==================
Bug fixes:

* [#32](https://github.com/helium/concentrate/pull/32):
  Drop CRC errored fragments from reassembly

0.2.1 (2019-08-07)
==================
Bug fixes:

* [#30](https://github.com/helium/concentrate/pull/30):
  Remove LongFi parsing timeouts

0.2.0 (2019-08-06)
==================
Bug fixes:

* [#29](https://github.com/helium/concentrate/pull/29):
  Fix LongFi [de]fragmentation.

0.1.9 (2019-07-28)
==================
Feature enhancements:

* [#26](https://github.com/helium/concentrate/pull/26):
  Add built in self test.

* [#23](https://github.com/helium/concentrate/pull/23):
  LongFi App for Concentrate.

0.1.8 (2019-07-23)
==================
Bug fixes:

* [#18](https://github.com/helium/concentrate/pull/18):
  Make calls to `libloragw-sys` threadsafe.

* [#21](https://github.com/helium/concentrate/pull/21):
  Add `.cargo/config` for cross-compilation. Extend `serve`/`listen` to allow `serve` to run on hardware but listener to be remote; `remote-ip` cli option was added to enable this.

* [#24](https://github.com/helium/concentrate/pull/24):
  Block in `transmit()` until Concentrator is able to accept a new TX packet.

0.1.7 (2019-06-26)
==================
Bug fixes:

* [#17](https://github.com/helium/concentrate/pull/17):
  Add lockfile to allow building with outdated Buildroot.

0.1.6 (2019-06-25)
==================
Feature enhancements:

* [#16](https://github.com/helium/concentrate/pull/16):
  Update reset line to use `/dev/gpio` symlink for portability.

0.1.5 (2019-06-21)
==================
Feature enhancements:

* [#15](https://github.com/helium/concentrate/pull/15):
  Add `syslog` logging backend.

0.1.4 (2019-06-19)
==================
Feature enhancements:

* [#14](https://github.com/helium/concentrate/pull/14):
  Switch to synchronous (req/resp) network interface.

0.1.3 (2019-06-17)
==================
Feature enhancements:

* [#11](https://github.com/helium/concentrate/pull/11):
  Allow enginering/scientific notation for frequency arguments.
* [#10](https://github.com/helium/concentrate/pull/10):
  Add implicit header flag to `send` command.

Bug fixes:

* [#13](https://github.com/helium/concentrate/pull/13):
  Lower SPI clock speed as it might be contributing to TX errors.

0.1.2 (2019-06-07)
==================
Feature enhancements:

* [#8](https://github.com/helium/concentrate/pull/8):
  Update channel scheme to be outside of LoRaWan.
* [#7](https://github.com/helium/concentrate/pull/7):
  Add transmit gain lookup-table to configuration.

0.1.1 (2019-06-03)
==================
Feature enhancements:

* [#4](https://github.com/helium/concentrate/pull/4):
  Improved error reporting on failed FFI calls.

Bug fixes:

* [#5](https://github.com/helium/concentrate/pull/5):
  Update RSSI offset per RAK2245/2247 spec.
