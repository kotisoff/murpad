use config::{Config, Value};

pub fn load_config() -> Config {
    let settings = Config::builder()
        .add_source(config::File::with_name("./Settings"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings
}

pub struct SoundField {
    pub label: String,
    pub file: String,
}

pub fn load_sounds() -> Vec<SoundField> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./Sounds"))
        .build()
        .unwrap();

    let sounds: Vec<SoundField> = settings
        .get_array("sounds")
        .unwrap()
        .iter()
        .map(|v| {
            let sound = v.clone().into_table().unwrap();

            SoundField {
                label: sound.get("label").unwrap().clone().into_string().unwrap(),
                file: sound.get("file").unwrap().clone().into_string().unwrap(),
            }
        })
        .collect();

    sounds
}

pub fn get_value(config: Config, category: &str, value: &str) -> Value {
    let val = config
        .get_table(category)
        .unwrap()
        .get(value)
        .unwrap()
        .clone();

    val
}
