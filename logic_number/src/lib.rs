pub fn number_logic(num: u32) -> bool {

    let l = num.to_string().len();
    let mut res = 0;
    let mut num1 = num.to_owned();
    while num1 > 0 {
        let digit: u32 = &num1%10;

        res += digit.pow(l as u32);

        num1 = num1/10;
    }
    num == res
}
