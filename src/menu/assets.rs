use std::fs;

pub const ASSET_PATH : &str = "./assets";

/// Reads a file from the asset-folder and returns its content
/// # Arguments
/// * `asset_name` is the name of the file, which should be read.
pub fn read_asset(asset_name: &str) -> String{

    let path = format!("{}/{asset_name}",ASSET_PATH);

    let contents = fs::read_to_string( path)
        .expect(format!("Should have been able to read the asset '{ASSET_PATH}/{}'",asset_name).as_str());
    contents
}
