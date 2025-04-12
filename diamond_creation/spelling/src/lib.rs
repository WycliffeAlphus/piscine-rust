pub fn spell(n: u64) -> String {
    const ONES: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine", "ten", "eleven", "twelve",
        "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen"
    ];

    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety"
    ];

    fn spell_hundreds(n: u64) -> String {
        let mut parts = vec![];

        if n >= 100 {
            parts.push(format!("{} hundred", ONES[(n / 100) as usize]));
        }

        let remainder = n % 100;

        if remainder > 0 {
            if remainder < 20 {
                parts.push(ONES[remainder as usize].to_string());
            } else {
                let tens = remainder / 10;
                let units = remainder % 10;

                if units == 0 {
                    parts.push(TENS[tens as usize].to_string());
                } else {
                    parts.push(format!("{}-{}", TENS[tens as usize], ONES[units as usize]));
                }
            }
        }

        parts.join(" ")
    }

    if n == 1_000_000 {
        return "one million".to_string();
    }

    let thousands = n / 1000;
    let below_thousand = n % 1000;

    let mut parts = vec![];

    if thousands > 0 {
        parts.push(format!("{} thousand", spell_hundreds(thousands)));
    }

    if below_thousand > 0 {
        parts.push(spell_hundreds(below_thousand));
    }

    if parts.is_empty() {
        "zero".to_string()
    } else {
        parts.join(" ").trim().to_string()
    }
}
