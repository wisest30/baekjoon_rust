use std::collections::HashMap;
use std::io::{Read, Write};

const INF: i64 = 1000000001;

#[derive(Default)]
struct PriceMap {
    changed: bool,
    map: HashMap<String, i64>,
}

impl PriceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_changed(&self) -> bool {
        self.changed
    }

    pub fn check_point(&mut self) {
        self.changed = false;
    }

    pub fn feed(&mut self, name: String, price: i64) {
        if price == -1 {
            return;
        }

        if !self.map.contains_key(&name) {
            self.map.insert(name, price);
            self.changed = true;
        } else {
            let prev_val = *self.map.get(&name).unwrap();
            if prev_val > price {
                self.map.insert(name, price);
                self.changed = true;
            }
        }
    }

    pub fn feed_recipe(&mut self, recipe: &Recipe) {
        let price = recipe.calculate_price(self);
        self.feed(recipe.name.clone(), price);
    }

    pub fn get_price(&self, name: &str) -> i64 {
        if self.map.contains_key(name) {
            *self.map.get(name).unwrap()
        } else {
            -1
        }
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    amount: i64,
}

impl Ingredient {
    pub fn new(s: &str) -> Self {
        let mut s = s.chars();
        let amount = s.next().unwrap().to_digit(10).unwrap() as i64;
        let name = s.collect::<String>();

        Self { name, amount }
    }

    pub fn calculate_price(&self, price_map: &PriceMap) -> i64 {
        let price = price_map.get_price(&self.name);
        if price == -1 {
            -1
        } else if price >= INF {
            INF
        } else {
            INF.min(self.amount * price)
        }
    }
}

struct Recipe {
    name: String,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(s: &str) -> Self {
        let s = s.split('=').collect::<Vec<_>>();
        let name = s[0];
        let s = s[1].split('+').collect::<Vec<_>>();
        let ingredients = s.iter().map(|s| Ingredient::new(s)).collect::<Vec<_>>();

        Self {
            name: name.to_string(),
            ingredients,
        }
    }

    pub fn calculate_price(&self, price_map: &PriceMap) -> i64 {
        let mut sum = 0;
        for ingredient in &self.ingredients {
            let price = ingredient.calculate_price(price_map);
            if price == -1 {
                return -1;
            } else if price >= INF {
                return INF;
            } else {
                sum += price;
            }
        }
        sum.min(INF)
    }
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut buf = String::new();

    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();

    let mut price_map = PriceMap::new();
    (0..n).for_each(|_| {
        let name = input.next().unwrap().to_string();
        let price = input.next().unwrap().parse::<i64>().unwrap();
        price_map.feed(name, price);
    });
    let recipes = (0..m)
        .map(|_| Recipe::new(input.next().unwrap()))
        .collect::<Vec<_>>();

    while price_map.get_changed() {
        price_map.check_point();
        for recipe in &recipes {
            price_map.feed_recipe(recipe);
        }
    }

    writeln!(stdout, "{}", price_map.get_price("LOVE")).ok();
}
