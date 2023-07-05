use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    vec,
};

use base64::{engine::general_purpose, Engine};
use id3::{
    frame::{Picture, PictureType}, Tag, TagLike,
};

use crate::{
    model::{Mata, Image},
    util::{
        aes::{self, decrypt_aes},
        cr4::CR4,
        get_length,
    },
};

pub fn ncm2mp3(src: String, dst: String) {
    let mut file = File::open(src).expect("is not a file");
    magic_header(&mut file).unwrap();
    let key = cr4_key(&mut file);
    let text = mata_data(&mut file);
    println!("{}", text);
    let mata: Mata = serde_json::from_str(text.as_str()).unwrap();
    let _image = album_image(&mut file);
    music_data(&mut file, key, &dst);
    combine(dst, mata, Image::from(_image));
}



fn combine(path: String, mata: Mata, image: Image) {
    let mut tag = Tag::read_from_path(&path).unwrap();
    tag.set_artist(mata.artist[0][0].as_str().unwrap());
    tag.set_album(mata.album);
    tag.add_frame(Picture {
        mime_type: image.get_format(),
        picture_type: PictureType::CoverFront,
        description: "".to_string(),
        data: image.bytes.clone(),
    });
    tag.add_frame(Picture {
        mime_type: "image/jpg".to_string(),
        picture_type: PictureType::Media,
        description: "".to_string(),
        data: image.bytes,
    });
    tag.write_to_path(path, id3::Version::Id3v22).unwrap()
}

fn music_data(file: &mut File, key: Vec<u8>, output: &String) {
    let mut cr4 = CR4::new();
    cr4.ksa(&key);
    let mut buffer = [0u8; 0x8000];
    let mut options = OpenOptions::new();
    options.create(true);
    let mut wfile = File::create(output).unwrap();
    loop {
        let len = file.read(&mut buffer).unwrap();
        if len <= 0 {
            break;
        }
        cr4.prga(&mut buffer, len);
        wfile.write_all(&mut buffer).unwrap();
    
    }
}

fn album_image(file: &mut File) -> Vec<u8> {
    let mut bytes = [0u8; 4];
    file.read(&mut bytes).unwrap();
    let len = get_length(&bytes).unwrap();
    let mut bytes = vec![0; len as usize];
    file.read(&mut bytes).unwrap();
    bytes
}

fn mata_data(file: &mut File) -> String {
    let mut bytes = [0u8; 4];
    file.read(&mut bytes).unwrap();
    let length = get_length(&bytes).unwrap();
    let mut bytes = vec![0u8; length as usize];
    file.read(&mut bytes).unwrap();
    let mut _bytes = [0u8; 9];
    file.read(&mut _bytes).unwrap();
    for i in 0..length {
        bytes[i as usize] ^= 0x63;
    }
    let temp = Vec::from(&bytes[22..]);
    let temp = general_purpose::STANDARD.decode(temp).unwrap();
    let temp = decrypt_aes(&temp, aes::MATA_KEY);
    String::from_utf8_lossy(&temp[6..]).into_owned()
}

fn cr4_key(file: &mut File) -> Vec<u8> {
    let mut bytes = [0u8; 4];
    file.read(&mut bytes).unwrap();
    let length = get_length(&bytes).unwrap();
    let mut bytes = vec![0u8; length as usize];
    file.read(&mut bytes).unwrap();
    for i in 0..length {
        bytes[i as usize] ^= 0x64;
    }
    let bytes: Vec<u8> = decrypt_aes(&bytes, aes::CORE_KEY);
    Vec::from(&bytes[17..])
}

fn magic_header(file: &mut File) -> Result<usize, std::io::Error> {
    let mut buf = [0u8; 10];
    file.read(&mut buf)
}

#[test]
fn test() {
    ncm2mp3(
        "H:\\CloudMusic\\VipSongsDownload\\a.ncm".to_string(),
        String::from("result.mp3"),
    );
}
