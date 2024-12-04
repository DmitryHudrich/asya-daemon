#[derive(macros::Property, Debug, PartialEq, Eq)]
#[property(name(ConfigProperty), derive(Debug, Clone))]
struct Config {
    #[property(default)]
    field1: i32,
    #[property(default)]
    field2: String,

    #[property(
        default(InnerConfigProperty { field1: Some(3), field2: Some(EnumType::Variant) }),
        use_type(InnerConfigProperty)
    )]
    field3: InnerConfig,
}

#[derive(macros::Property, PartialEq, Eq, Debug)]
#[property(name(InnerConfigProperty), derive(Debug, Clone))]
struct InnerConfig {
    #[property(default(4))]
    field1: u8,
    #[property(default)]
    field2: EnumType,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum EnumType {
    #[default]
    Variant,
    Invariant,
}

#[test]
fn create_and_compare() {
    let mut inner_config_property = InnerConfigProperty {
        field1: Some(3),
        field2: EnumType::Variant.into(),
    };

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 3,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field1 = None;

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Variant
        }
    );

    let mut config_property = ConfigProperty {
        field1: None,
        field2: Some("hell".to_string()),
        field3: None,
    };

    assert_eq!(
        config_property.clone().unwrap_or_default(),
        Config {
            field1: 0,
            field2: "hell".to_string(),
            field3: InnerConfig {
                field1: 3,
                field2: EnumType::Variant
            }
        }
    );

    inner_config_property.field2 = EnumType::Invariant.into();
    config_property.field3 = Some(inner_config_property.clone());
    assert_eq!(
        config_property.clone().unwrap_or_default(),
        Config {
            field1: 0,
            field2: "hell".to_string(),
            field3: InnerConfig {
                field1: 4,
                field2: EnumType::Invariant,
            },
        },
    );
}

#[test]
fn correct_unwrap_or_default() {
    let mut inner_config_property = InnerConfigProperty {
        field1: None,
        field2: None,
    };

    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field1 = Some(2);
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 2,
            field2: EnumType::Variant
        }
    );

    inner_config_property.field2 = EnumType::Invariant.into();
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 2,
            field2: EnumType::Invariant
        }
    );

    inner_config_property.field1 = None;
    assert_eq!(
        inner_config_property.clone().unwrap_or_default(),
        InnerConfig {
            field1: 4,
            field2: EnumType::Invariant
        }
    );
}
