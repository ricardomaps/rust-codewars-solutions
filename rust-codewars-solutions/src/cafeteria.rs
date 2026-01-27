#[derive(Debug)]
pub struct Coffee {
    pub sort: String,
    pub milk: Vec<Milk>,
    pub sugar: Vec<Sugar>,
}

#[derive(Debug)]
pub struct Milk {
    pub fat: f32,
}

#[derive(Debug)]
pub struct Sugar {
    sort: String,
}

struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: "".to_owned(),
            milk: vec![],
            sugar: vec![],
        }
    }

    fn set_black_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Black".to_owned();
        self
    }

    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Cubano".to_owned();
        self.sugar.push(Sugar {
            sort: "Brown".to_owned(),
        });
        self
    }

    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Americano".to_owned();
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk { fat });
        self
    }

    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        self.sugar.push(Sugar { sort });
        self
    }

    fn build(self) -> Coffee {
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar,
        }
    }
}
