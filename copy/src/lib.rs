pub fn nbr_function(c: i32) -> (i32, f64, f64) {

(c, (c as f64).exp(), (c as f64).abs().ln())

}

pub fn str_function(a: String) -> (String, String) {

let b = a.chars();

let mut vecs:Vec<String> = Vec::new(); 

for ex in b {
    (ex as u8 as f64).exp().to_string();
    vecs.push(ex.to_string());
}
let s = vecs.join(",");
(a, s)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {

    let mut vecs: Vec<f64> = Vec::new();

    for v in &b {
        vecs.push((*v as f64).abs().ln())
    }

    (b,vecs)

}