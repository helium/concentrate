use serde::{Deserialize, Serialize};

static DEFAULT_CFG_TOML: &str = include_str!("../../default_config.toml");

/// Represents top-level configuration document.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Config {
    lorawan_public: bool,
    clksrc: u8,
    radios: Option<Vec<Radio>>,
    multirate_channels: Option<Vec<MultirateLoraChannel>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Radio {
    id: u8,
    freq: u32,
    rssi_offset: f32,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_: String,
    tx_enable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct MultirateLoraChannel {
    id: u8,
    radio: u8,
    #[serde(rename(serialize = "if", deserialize = "if"))]
    if_: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_deserialize_radioconfig() {
        let parsed_config = toml::from_str::<Config>(DEFAULT_CFG_TOML).unwrap();
        let reference_config = Config {
            lorawan_public: false,
            clksrc: 1,
            radios: Some(vec![
                Radio {
                    id: 0,
                    freq: 867_500_000,
                    rssi_offset: -166.0,
                    type_: "SX1257".to_string(),
                    tx_enable: false,
                },
                Radio {
                    id: 1,
                    freq: 868_500_000,
                    rssi_offset: -166.0,
                    type_: "SX1257".to_string(),
                    tx_enable: false,
                },
            ]),
            multirate_channels: Some(vec![
                MultirateLoraChannel {
                    id: 0,
                    radio: 1,
                    if_: -400_000,
                },
                MultirateLoraChannel {
                    id: 1,
                    radio: 1,
                    if_: -200_000,
                },
                MultirateLoraChannel {
                    id: 2,
                    radio: 1,
                    if_: 0,
                },
                MultirateLoraChannel {
                    id: 3,
                    radio: 0,
                    if_: -400_000,
                },
                MultirateLoraChannel {
                    id: 4,
                    radio: 0,
                    if_: -200_000,
                },
                MultirateLoraChannel {
                    id: 5,
                    radio: 0,
                    if_: 0,
                },
                MultirateLoraChannel {
                    id: 6,
                    radio: 0,
                    if_: 200_000,
                },
                MultirateLoraChannel {
                    id: 7,
                    radio: 0,
                    if_: 400_000,
                },
            ]),
        };
        assert_eq!(reference_config, parsed_config);
    }
}
