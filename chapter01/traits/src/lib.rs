use std::io::{Read, Write};

pub struct Config {
    values: Vec<(String, String)>,
}

pub struct KeyValueConfigService {}

impl Config {
    pub fn new(values: Vec<(String, String)>) -> Config {
        Config { values: values }
    }
}

impl KeyValueConfigService {
    pub fn new() -> KeyValueConfigService {
        KeyValueConfigService {}
    }
}

pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

pub trait ConfigWriter {
    fn write(&self, config: Config, to: &mut impl Write) -> std::io::Result<()>;
}

pub trait ConfigReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config>;
}

impl ConfigWriter for KeyValueConfigService {
    fn write(&self, config: Config, mut to: &mut impl Write) -> std::io::Result<()> {
        for v in config.values {
            writeln!(&mut to, "{0}={1}", v.0, v.1)?;
        }
        Ok(())
    }
}

impl ConfigReader for KeyValueConfigService {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Config> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?;

        // chain iterations together and collect the results
        let values: Vec<(String, String)> = buffer
            .split_terminator("\n")
            .map(|line| line.trim())
            .filter(|line| {
                let pos = line.find("=").unwrap_or(0);
                pos > 0 && pos < line.len() - 1
            })
            .map(|line| {
                let parts = line.split("=").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect();
        Ok(Config::new(values))
    }
}

impl ValueGetter for Config {
    fn get(&self, s: &str) -> Option<String> {
        self.values.iter().find_map(|tuple| {
            if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
