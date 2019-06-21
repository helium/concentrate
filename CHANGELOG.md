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
