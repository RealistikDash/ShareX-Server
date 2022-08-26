use rand::{distributions::Alphanumeric, distributions::DistString};
use std::path::Path;
use actix_web::web::Bytes;
use crate::sharex::users::User;

pub enum ImageTypes {
    PNG,
    JPG,
    GIF,
}

impl ImageTypes {
    pub fn into_extension(&self) -> &str {
        match self {
            Self::PNG => "png",
            Self::GIF => "gif",
            Self::JPG => "jpg",
        }
    }
}

fn random_string(size: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), size)
}

const NAME_LEN: usize = 8;

/// Generates a unique filename, ensuring no files are overwritten.
pub fn generate_unique_filename(save_path: String, extension: ImageTypes) -> String {
    loop {
        let file_name = format!("{}.{}", random_string(NAME_LEN), extension.into_extension());
        let file_path = format!("{}/{}", save_path, file_name);
        let path = Path::new(&file_path);

        if !path.exists() {
            return path.to_str().unwrap().to_string();
        }
    }
}

const PNG_SLICE: &'static [u8] = b"\x89PNG\r\n\x1a\n";
const JPG_SLICE: &'static [u8] = b"JFIF";
const GIF_SLICE: &'static [u8] = b"GIF8";

fn detect_type(file: Vec<u8>) -> Option<ImageTypes> {
    if &file[..8] == PNG_SLICE {
        Some(ImageTypes::PNG)
    } else if &file[6..10] == JPG_SLICE {
        Some(ImageTypes::JPG)
    }  else if &file[..4] == GIF_SLICE {
        Some(ImageTypes::GIF)
    } else {
        None
    }
}

pub fn upload_image(user: &User, image: &Bytes) -> Result<String, ()> {
    
}
