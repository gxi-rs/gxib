use toml::Value;
use toml::value::Table;

const DEPENDENCIES_STR: &str = "dependencies";
const VERSION_STR: &str = "version";

pub struct CargoToml {
    toml: Value,
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
                    table.entry(VERSION_STR).or_insert(Value::String(String::from(str)));
                    table
                });
                &mut dependency_table["gxi"]
            } else {
                gxi_table
            };
            gxi_table.as_table_mut().unwrap()
        };
        //check props
        {
            gxi_table.entry("version").or_insert_with(|| Value::String(String::new()));
        }
    }
    CargoToml {
        toml: cargo_toml
    }
}

#[test]
fn test_parse_cargo_toml() {
    //no dependency
    {
        let cargo_toml = parse_cargo_toml("".as_bytes());
        assert_eq!(cargo_toml.toml.to_string(), format!("[{}.gxi]\nversion = \"\"\n", DEPENDENCIES_STR))
    }
    {
        let test_str = format!("[{dep}]\nk = \"\"\n\n[{dep}.gxi]\nversion = \"0.0.1\"\n", dep = DEPENDENCIES_STR);

        //with dependency
        {
            let cargo_toml = parse_cargo_toml(format!(r#"
                [{}]
                k = ""
                gxi = "0.0.1"
            "#, DEPENDENCIES_STR).as_bytes());
            assert_eq!(cargo_toml.toml.to_string(), test_str);
        }
        {
            let cargo_toml = parse_cargo_toml(format!(r#"
                [{}]
                k = ""
                gxi = {{ version = "0.0.1" }}
            "#, DEPENDENCIES_STR).as_bytes());
            assert_eq!(cargo_toml.toml.to_string(), test_str);
        }
    }
    {
        let cargo_toml = parse_cargo_toml(format!(r#"
            [{}.gxi]
            version = "0.0.1"
        "#, DEPENDENCIES_STR).as_bytes());
        assert_eq!(cargo_toml.toml.to_string(), format!("[{dep}.gxi]\nversion = \"0.0.1\"\n", dep = DEPENDENCIES_STR))
    }
}