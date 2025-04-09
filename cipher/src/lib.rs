#[derive(Debug, PartialEq)]
pub struct CipherError {
pub expected: String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {

    let expected = atbash(original);
    if ciphered.to_string() == expected {
        Ok(())
    } else {
        Err(CipherError {expected})
    }

}

fn atbash(input: &str) -> String {
    input
        .chars()
        .map(|c|{
            if c.is_ascii_uppercase(){
                (b'Z' - (c as u8 - b'A')) as char
            } else if c.is_ascii_lowercase(){
                (b'z' - (c as u8 - b'a')) as char
            } else {
                c
            }
        })
        .collect()
}