use anyhow::Result;
use std::fs::File;
use std::io::{Read,Seek};
use serde::Deserialize;
use regex::Regex;

pub struct ArtifactQuery {
	magic: Vec<artifact>,
}

impl ArtifactQuery {
	pub fn new() -> Result<Self>{
		let magic = magic_list()?;
		Ok(Self{
			magic
		})
	}

   	pub fn from_path(&self, path: &str) -> Result<Option<String>>{
		let mut file = File::open(path)?;
		self.from_reader(&mut file)
	}

	pub fn from_reader<R: Read + Seek>(&self, reader: &mut R) -> Result<Option<String>>{
		let magic_ls = &self.magic;
		let mut data = vec![0;32];
		let _ = reader.read_exact(&mut data)?;

		let mut result_artifact: Option<String> = None;

		for art in magic_ls.iter(){
			let len_magic_sig = art.clone().signature.split(" ").collect::<Vec<&str>>().len();
			let nospace_magic_sig = art.clone().signature.replace(" ","").replace("?","[a-f0-9]");
			let format_regex = format!("^{}$",nospace_magic_sig);
			
			let exact_data = &data[..len_magic_sig];
			let data_hex = hex::encode(exact_data);
			
			let re = Regex::new(&format_regex).unwrap();
			if re.is_match(&data_hex) {
				result_artifact = Some(art.clone().name);
				return Ok(result_artifact)
			}
		}
		Ok(result_artifact)
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Magic {
	magics: Vec<artifact>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct artifact {
	pub name: String,
	pub signature: String,
}

fn magic_list() -> Result<Vec<artifact>> {
	let data = include_bytes!("../magic.json");
    let magics: Magic = serde_json::from_slice(data)?;
    Ok(magics.magics.clone())
}