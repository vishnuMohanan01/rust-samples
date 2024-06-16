mod back_of_house;

use self::back_of_house::hosting;

pub fn eat_at_restaurant() {
  let meal_1 = hosting::Breakfast::summer("bread");
  
  let meal_2 = hosting::Breakfast {
    toast: String::from("bread_2"),
    seasonal_fruit: String::from("pinepple"),
  };
}
