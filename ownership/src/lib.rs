pub fn first_subword(mut s: String) -> String {

    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate(){
        

        if item == b'_' || (item as char).is_uppercase()&&i>0{
            s.truncate(i);
            return s;
        }
    }
s
}
