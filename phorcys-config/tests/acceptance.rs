//! Tests the configuration acceptance.

use phorcys_config::vrchat::config::{
    Configuration, Parameter, ParameterAddress, ParameterDataType,
};
use serde_json::from_str;

#[test]
fn test_config_structure() {
    let config_source = include_str!("./parameters.json");
    let config: Configuration = from_str(config_source).expect("Invalid form");

    assert_eq!(config.id, "avtr_00000000-0000-0000-0000-000000000000");
    assert_eq!(config.name, "セフィラちゃん（夏稀風）");
    assert_eq!(config.parameters.len(), 42);
    assert_eq!(
        config.parameters[0],
        Parameter {
            name: "VRCLFeatureToggle".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/VRCLFeatureToggle".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/VRCLFeatureToggle".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[1],
        Parameter {
            name: "VRCFaceBlendV".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/VRCFaceBlendV".into(),
                parameter_type: ParameterDataType::Float,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/VRCFaceBlendV".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[2],
        Parameter {
            name: "VRCFaceBlendH".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/VRCFaceBlendH".into(),
                parameter_type: ParameterDataType::Float,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/VRCFaceBlendH".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[3],
        Parameter {
            name: "Onegai".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Onegai".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Onegai".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[4],
        Parameter {
            name: "Boobs".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Boobs".into(),
                parameter_type: ParameterDataType::Float,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Boobs".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[5],
        Parameter {
            name: "Face Shocked".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Shocked".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Shocked".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[6],
        Parameter {
            name: "Face Spirit".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Spirit".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Spirit".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[7],
        Parameter {
            name: "Face Blue".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Blue".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Blue".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[8],
        Parameter {
            name: "Face Tears".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Tears".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Tears".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[9],
        Parameter {
            name: "Face Shy2".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Shy2".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Shy2".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[10],
        Parameter {
            name: "Face Shy1".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Shy1".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Shy1".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[11],
        Parameter {
            name: "Face Cheeks".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Face Cheeks".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Face Cheeks".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[12],
        Parameter {
            name: "Mouth".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Mouth".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Mouth".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[13],
        Parameter {
            name: "Pupils".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Pupils".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Pupils".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[14],
        Parameter {
            name: "Eyelids".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Eyelids".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Eyelids".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[15],
        Parameter {
            name: "Eyebrows".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Eyebrows".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Eyebrows".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[16],
        Parameter {
            name: "Item RickRoll".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Item RickRoll".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Item RickRoll".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[17],
        Parameter {
            name: "Item Right".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Item Right".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Item Right".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[18],
        Parameter {
            name: "Clothes NoBag".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Clothes NoBag".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Clothes NoBag".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[19],
        Parameter {
            name: "Clothes Legs".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Clothes Legs".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Clothes Legs".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[20],
        Parameter {
            name: "Clothes Base".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/Clothes Base".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/Clothes Base".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[21],
        Parameter {
            name: "System HandAnimated".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/System HandAnimated".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/System HandAnimated".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[22],
        Parameter {
            name: "System FaceMirror".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/System FaceMirror".into(),
                parameter_type: ParameterDataType::Bool,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/System FaceMirror".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[23],
        Parameter {
            name: "VRCEmote".into(),
            input: Some(ParameterAddress {
                address: "/avatar/parameters/VRCEmote".into(),
                parameter_type: ParameterDataType::Int,
            }),
            output: ParameterAddress {
                address: "/avatar/parameters/VRCEmote".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[24],
        Parameter {
            name: "VelocityZ".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/VelocityZ".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[25],
        Parameter {
            name: "VelocityY".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/VelocityY".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[26],
        Parameter {
            name: "VelocityX".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/VelocityX".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[27],
        Parameter {
            name: "InStation".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/InStation".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[28],
        Parameter {
            name: "Seated".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/Seated".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[29],
        Parameter {
            name: "AFK".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/AFK".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[30],
        Parameter {
            name: "Upright".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/Upright".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[31],
        Parameter {
            name: "AngularY".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/AngularY".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[32],
        Parameter {
            name: "Grounded".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/Grounded".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[33],
        Parameter {
            name: "MuteSelf".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/MuteSelf".into(),
                parameter_type: ParameterDataType::Bool,
            }
        }
    );
    assert_eq!(
        config.parameters[34],
        Parameter {
            name: "VRMode".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/VRMode".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[35],
        Parameter {
            name: "TrackingType".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/TrackingType".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[36],
        Parameter {
            name: "GestureRightWeight".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/GestureRightWeight".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[37],
        Parameter {
            name: "GestureRight".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/GestureRight".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[38],
        Parameter {
            name: "GestureLeftWeight".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/GestureLeftWeight".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[39],
        Parameter {
            name: "GestureLeft".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/GestureLeft".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
    assert_eq!(
        config.parameters[40],
        Parameter {
            name: "Voice".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/Voice".into(),
                parameter_type: ParameterDataType::Float,
            }
        }
    );
    assert_eq!(
        config.parameters[41],
        Parameter {
            name: "Viseme".into(),
            input: None,
            output: ParameterAddress {
                address: "/avatar/parameters/Viseme".into(),
                parameter_type: ParameterDataType::Int,
            }
        }
    );
}
