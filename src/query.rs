use tokio::io::{AsyncReadExt, AsyncSeekExt};
use anyhow::Result;
use tokio::fs::File;
use serde::Deserialize;
use regex::Regex;

pub struct ArtifactQuery {}

impl ArtifactQuery {
   	pub async fn from_path(path: &str) -> Result<Option<String>>{
		let mut file = File::open(path).await?;
		Self::from_reader(&mut file).await
	}

	pub async fn from_reader<R: AsyncReadExt + AsyncSeekExt + Unpin>(reader: &mut R) -> Result<Option<String>>{
		let magic_ls = magic_list().await?;
		let mut data = vec![0;32];
		let _ = reader.read_exact(&mut data).await?;

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

async fn magic_list() -> Result<Vec<artifact>> {
	let mut magic_file = File::open("magic.json").await?;
	let mut data = Vec::new();
	let _ = magic_file.read_to_end(&mut data).await?;
	let magics: Magic = serde_json::from_slice(&data)?;
	Ok(magics.magics.clone())
}