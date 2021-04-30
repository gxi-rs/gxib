use toml::Value;
use toml::value::Table;

const DEPENDENCIES_STR: &str = "dependencies";
const VERSION_STR: &str = "version";
const FEATURES_STR: &str = "features";

pub struct CargoToml(Value);

impl ToString for CargoToml {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub fn parse_cargo_toml(bytes: &[u8]) -> CargoToml {
    let mut cargo_toml: Value = toml::from_slice(bytes).unwrap();
    {
        let cargo_toml = cargo_toml.as_table_mut().unwrap();
        // get the [dependency] part
        let dependency_table = {
            let dependency_table = cargo_toml
                .entry(DEPENDENCIES_STR)
                .or_insert_with(|| Value::Table(Table::new()));
            dependency_table.as_table_mut().unwrap()
        };
        // get the gxi dependency
        let gxi_table = {
            let gxi_table = dependency_table
                .entry("gxi")
                .or_insert_with(|| Value::Table(Table::new()));
            // check if the value is a string
            let gxi_table = if let Some(str) = gxi_table.as_str() {
                // if it is a string then convert it to a table
                // and move that string to as its version
                dependency_table["gxi"] = Value::Table({
                    let mut table = Table::new();
                    table
                        .entry(VERSION_STR)
                        .or_insert(Value::String(String::from(str)));
                    table
                });
                &mut dependency_table["gxi"]
            } else {
                gxi_table
            };
            gxi_table
                .as_table_mut()
                .expect("Expected table as {} or string as \"\" for the value of dependency gxi")
        };
        //check props
        {
            // check version
            gxi_table
                .entry(VERSION_STR)
                .or_insert_with(|| Value::String(String::new()));
            // check features
            {
                let features = {
                    let features = gxi_table
                        .entry(FEATURES_STR)
                        .or_insert_with(|| Value::Array(Vec::new()));
                    features.as_array_mut().expect(
                        "Expected array of strings as the value of features of dependency gxi",
                    )
                };
                // remove both web and desktop feature
                // because they'll be automatically be added by
                // the respected pipelines
                {
                    let mut to_remove = Vec::new();
                    for (i, val) in features.iter().enumerate() {
                        let str = val.as_str().expect("Values of feature array can only have strings");
                        if str == "desktop" || str == "web" {
                            // features resizes when an element is removed so the index should be shifted
                            to_remove.push(i - to_remove.len());
                        }
                    }
                    for x in to_remove {
                        features.remove(x);
                    }
                }
            }
        }
    }
    CargoToml(cargo_toml)
}

#[test]
fn test_parse_cargo_toml() {
    //no dependency
    {
        let cargo_toml = parse_cargo_toml("".as_bytes());
        assert_eq!(
            cargo_toml.to_string(),
            format!(
                "[{}.gxi]\n{} = \"\"\n{} = []\n",
                DEPENDENCIES_STR, VERSION_STR, FEATURES_STR
            )
        )
    }
    {
        let test_str = format!(
            "[{dep}]\nk = \"\"\n\n[{dep}.gxi]\n{ver} = \"\"\n{fea} = []\n",
            dep = DEPENDENCIES_STR,
            ver = VERSION_STR,
            fea = FEATURES_STR
        );
        //with dependency
        {
            let cargo_toml = parse_cargo_toml(
                format!(
                    r#"
                        [{}]
                        k = ""
                        gxi = ""
                    "#,
                    DEPENDENCIES_STR
                )
                    .as_bytes(),
            );
            assert_eq!(cargo_toml.to_string(), test_str);
        }
        {
            let cargo_toml = parse_cargo_toml(
                format!(
                    r#"
                        [{}]
                        k = ""
                        gxi = {{ {} = "" }}
                    "#,
                    DEPENDENCIES_STR, VERSION_STR
                )
                    .as_bytes(),
            );
            assert_eq!(cargo_toml.to_string(), test_str);
        }
        {
            let cargo_toml = parse_cargo_toml(
                format!(
                    r#"
                        [{}]
                        k = ""
                        gxi = {{}}
                    "#,
                    DEPENDENCIES_STR
                )
                    .as_bytes(),
            );
            assert_eq!(cargo_toml.to_string(), test_str);
        }
    }
    // check extra props
    {
        let test_str = format!(
            "[{}.gxi]\n{} = \"0.0.1\"\nhello = [\"foo\"]\n{} = []\n",
            DEPENDENCIES_STR, VERSION_STR, FEATURES_STR
        );
        {
            let cargo_toml = parse_cargo_toml(
                format!(
                    r#"
                        [{}.gxi]
                        {} = "0.0.1"
                        hello = [ "foo" ]
                    "#,
                    DEPENDENCIES_STR, VERSION_STR
                )
                    .as_bytes(),
            );
            assert_eq!(cargo_toml.to_string(), test_str);
        }
        {
            let cargo_toml = parse_cargo_toml(
                format!(
                    r#"
                        [{}]
                        gxi = {{ {} = "0.0.1", hello = [ "foo" ] }}
                    "#,
                    DEPENDENCIES_STR, VERSION_STR
                )
                    .as_bytes(),
            );
            assert_eq!(cargo_toml.to_string(), test_str);
        }
    }
    // features check
    {
        let cargo_toml = parse_cargo_toml(
            format!(
                r#"
                    [{}.gxi]
                    features = ["desktop","web","async","web","desktop"]
                "#,
                DEPENDENCIES_STR
            )
                .as_bytes(),
        );
        assert_eq!(cargo_toml.to_string(), format!(
            "[{}.gxi]\n{} = [\"async\"]\n{} = \"\"\n",
            DEPENDENCIES_STR, FEATURES_STR, VERSION_STR
        ));
    }
}
