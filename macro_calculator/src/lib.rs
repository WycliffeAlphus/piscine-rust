use json;
use std::str::FromStr;

pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut total_calories = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;

    for food in foods {
        
        let calories_str = &food.calories[1];
        let calories_value = extract_numeric_value(calories_str);
        
        total_calories += calories_value * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
    }


    json::object! {
        "cals": format_value(total_calories),
        "carbs": format_value(total_carbs),
        "proteins": format_value(total_proteins),
        "fats": format_value(total_fats),
    }
}

// Helper function to extract numeric value from a string like "510kcal"
fn extract_numeric_value(s: &str) -> f64 {
    let numeric_part: String = s.chars()
        .take_while(|c| c.is_digit(10) || *c == '.')
        .collect();
    
    f64::from_str(&numeric_part).unwrap_or(0.0)
}


fn format_value(value: f64) -> f64 {
   
    let rounded = (value * 100.0).round() / 100.0;
    
 
    if (rounded * 10.0).round() / 10.0 == rounded {
      
        (value * 10.0).round() / 10.0
    } else {
       
        rounded
    }
}