pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.as_bytes().iter().all(|&b| b <= 127)
}

pub fn contains(v: &str, pat: &str) -> bool {

if pat.len() == 0 {
    return true;
}
 
let v_bytes = v.as_bytes();
let pat_bytes = pat.as_bytes();

if pat_bytes.len() > v_bytes.len(){
    return false;
}

for i in 0..= v_bytes.len() - pat_bytes.len(){
    if v_bytes[i..i+pat_bytes.len()].eq(pat_bytes){
        return true;
    }
}
return false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.chars().position(|p| p == pat).unwrap_or(usize::MAX)
}
