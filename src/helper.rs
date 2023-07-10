use bytes::{Buf, BytesMut}; // get_u8

pub fn buffer_to_array(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = vec![];
    let length = buf.len();
    let mut word = "".to_string();

    for i in 0..length {
        match buf.get_u8() {
            b' ' => {
                // match for space
                vec.push(word);
                word = "".to_string();
            }
            other => {
                // increase the word
                word.push(other as char);
                let new = word.clone();
                if i == length - 1 {
                    vec.push(new);
                }
            }
        }
    }
    vec
}
