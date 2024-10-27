use askama::Template;
use fraction::Fraction;
use std::{fmt::{self, write}, fs, path::Display};
use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Serialize, Deserialize, Template)]
#[template(path = "ingredient.html")]
struct Ingredient {
    name: String,
    quantity: Fraction,
    unit: String,
    note: Option<String>,
    substitute: Option<Box<Ingredient>>
}

#[derive(Serialize, Deserialize, Template)]
#[template(path = "recipe.html")]
struct Recipe {
    ingredients: Vec<Ingredient>,
}

fn render_recipe() -> Result<(), Box<dyn std::error::Error>>  {
    let recipe = fs::read_to_string("recipes/simple.yaml")?;
    let recipe: Recipe = serde_yaml::from_str(&recipe)?;
    let json = serde_json::to_string(&recipe).unwrap();
    fs::write("recipe.json", json);
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