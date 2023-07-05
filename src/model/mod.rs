use serde::Deserialize;
use serde_json::Value;



#[derive(Deserialize)]
pub struct Mata {
    /**
     * 音乐名
     */
    pub musicName:String,
    /**
     * 艺术家
     */
    pub artist:Vec<Vec<Value>>,
    /**
     * 专辑
     */
    pub album:String,

    /**
     * 格式
     */
    pub format:String,
}

pub struct Image{
    pub bytes:Vec<u8>,
}

impl Image{
    pub fn from(vec:Vec<u8>) -> Image {
        Image{
            bytes:vec
        }
    }

    pub fn get_format(&self)->String {
        let h_png:[u8;8] =  [ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        if self.bytes[0..8] == h_png[..] {
            return "image/png".to_string()
        }
        "image/jpg".to_string()
        // PNG file header
    }
}