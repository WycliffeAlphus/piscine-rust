pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {

   let mut res = Vec::<u32>::new();
   let mut new = s.to_owned();
    if new.ends_with('k'){
        new.pop();
        res.push(s.parse::<u32>().unwrap() * (1000 as u32));
        return Box::new(res);
    }

    res.push(new.parse::<u32>().unwrap());

    Box::new(res)
}



pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {


    *a

}