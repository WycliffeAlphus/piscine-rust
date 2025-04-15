pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut res = Vec::<u32>::new();
    let splitted: Vec<&str> = s.split_whitespace().collect();

    for word in splitted {
        if word.ends_with('k') {
          
            let mut temp = word.to_string(); 
            temp.pop();                      
            let num = temp.parse::<f64>().unwrap(); 
            res.push((num * 1000.0) as u32);
        } else {
            let num = word.parse::<f64>().unwrap();
            res.push((num)as u32);
        }
    }

    Box::new(res)
}



pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {


    *a

}