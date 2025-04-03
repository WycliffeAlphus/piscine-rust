pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.as_bytes().iter().all(|&b| b <= 127)
}

pub fn contains(v: &str, pat: &str) -> bool {
  for item in v.chars() {
    if item == pat {
        return true;
    }
  }
  return false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    v.chars().position(|p| p == pat)
}
