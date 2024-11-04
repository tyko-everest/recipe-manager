use askama::Template;
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{fmt::Display, fs, usize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Volume {
    Teaspoon,
    Tablespoon,
    Cup,
    Milliliter,
    Liter,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Mass {
    Gram,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Scalable {
    Pinch,
    Can,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
enum Unit {
    Volume(Volume),
    Mass(Mass),
    Scalable(Scalable),
    Taste,
}

#[derive(Serialize, Deserialize, Clone)]
struct Amount {
    quantity: Fraction,
    unit: Unit,
}

impl Amount {
    fn simplify(&mut self) {
        match &self.unit {
            Unit::Volume(volume) => match &volume {
                Volume::Teaspoon => {
                    if self.quantity >= Fraction::from(3) {
                        self.quantity /= 3;
                        self.unit = Unit::Volume(Volume::Tablespoon);
                        self.simplify();
                    }
                }
                Volume::Tablespoon => {
                    if self.quantity >= Fraction::from(4) && self.quantity % 4 == Fraction::from(0)
                    {
                        self.quantity /= 16;
                        self.unit = Unit::Volume(Volume::Cup);
                        self.simplify();
                    }
                }
                Volume::Cup => {}
                Volume::Milliliter => todo!(),
                Volume::Liter => {}
            },
            Unit::Mass(mass) => match &mass {
                Mass::Gram => {}
            },
            Unit::Scalable(_) => {}
            Unit::Taste => {}
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Ingredient {
    name: String,
    amount: Amount,
    note: Option<String>,
    substitute: Option<Box<Ingredient>>,
}

impl Ingredient {
    fn scale(factor: usize) -> Self {
        todo!()
    }
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Template)]
#[template(path = "recipe.html")]
struct Recipe {
    ingredients: Vec<Ingredient>,
}

fn render_recipe() -> Result<(), Box<dyn std::error::Error>> {
    let recipe = fs::read_to_string("recipes/simple.yaml")?;
    let recipe: Recipe = serde_yaml::from_str(&recipe)?;
    let json = serde_json::to_string(&recipe).unwrap();
    fs::write("recipe.json", json).unwrap();
    let text = recipe.render().unwrap();
    fs::write("out.html", text).unwrap();
    Ok(())
}

fn main() {
    let res = render_recipe();
    match res {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}

#[test]
fn test_conversion() {
    let mut test = Amount {
        quantity: Fraction::from(3),
        unit: Unit::Volume(Volume::Teaspoon),
    };

    test.simplify();

    assert_eq!(test.quantity, Fraction::from(1));
    assert_eq!(test.unit, Unit::Volume(Volume::Tablespoon));
}
