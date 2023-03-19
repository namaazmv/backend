use eyre::Result;
use serde::de::DeserializeOwned;
use std::fs::File;

pub fn convert_csv<D>(name: String) -> Result<Vec<D>>
where
    D: DeserializeOwned,
{
    let file = File::open(format!("assets/{}.csv", &name))
        .unwrap_or_else(|_| panic!("Failed to open {}.csv", name));
    let mut reader = csv::ReaderBuilder::new().from_reader(file);
    let mut contents = vec![];

    for content in reader.deserialize::<D>() {
        contents.push(content?);
    }

    Ok(contents)
}
