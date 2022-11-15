use serde::{Deserialize, Serialize};
use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
//use actix_web::web::Path;

use crate::response::Response;
use crate::ingredient::Ingredient;

pub type Recipes = Response<Recipe>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub id: String,
    pub created_at: String,
    pub instructions: Vec<String>,
    pub body: String,
    pub ingredients: Vec<Ingredient>,
    pub url: String,
}

impl Recipe {
    pub fn new(instructions: Vec<String>, body: String, ingredients: Vec<Ingredient>, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: chrono::offset::Utc::now().to_string(),
            instructions,
            body,
            ingredients,
            url,
        }
    }
}

/*
#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeRequest {
    pub fn to_recipe(&self) -> Option<Recipe> {
        match &self.
    }
}
*/

#[get("/recipes")]
pub async fn list() -> HttpResponse {
    // TODO: get recipes this will have query params "?ingredients=apple,chicken,thyme"
    let r1 = Recipe::new(
        vec![
            "1. put your right foot in".to_string(),
            "2. put your right out".to_string(),
            "3. then you shake it all about".to_string()
        ],
        "this is the body of the recipe. You can put whatever you want here".to_string(),
        vec![
            Ingredient::new("apple".to_string()),
            Ingredient::new("chicken".to_string()),
            Ingredient::new("thyme".to_string()),
        ],
        "www.spacekimchis.net".to_string(),
    );
    let r2 = Recipe::new(
        vec![
            "1. second recip instructions".to_string(),
            "2. loren ipsum".to_string(),
            "3. trident".to_string()
        ],
        "this is the body of the recipe. You can put whatever you want here".to_string(),
        vec![
            Ingredient::new("ribeye".to_string()),
            Ingredient::new("butter".to_string()),
            Ingredient::new("corn".to_string()),
        ],
        "www.spacekimchis.net".to_string(),
    );
    let recipes = Recipes { results: vec![r1, r2] };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(recipes)
}

#[post("/recipes")]
pub async fn create(recipe_req: Json<Recipe>) -> HttpResponse {
    let r1 = Recipe::new(
        vec![
            "1. put your right foot in".to_string(),
            "2. put your right out".to_string(),
            "3. then you shake it all about".to_string()
        ],
        "this is the body of the recipe. You can put whatever you want here".to_string(),
        vec![
            Ingredient::new("apple".to_string()),
            Ingredient::new("chicken".to_string()),
            Ingredient::new("thyme".to_string()),
        ],
        "www.spacekimchis.net".to_string(),
    );
    HttpResponse::Created()
        .content_type("application/json")
        .json(r1)
}

#[get("/recipes/{recipe_id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    // TODO: Get recipe by ID. This will discard query params
    let found_recipe: Option<Recipe> = None;

    match found_recipe {
        Some(recipe) => HttpResponse::Ok()
            .content_type("application/json")
            .json(recipe),
        None => HttpResponse::NoContent()
            .content_type("application/json")
            .await
            .unwrap(),
    }
}

#[delete("/recipes/{recipe_id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    // TODO: Delete recipe by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type("application/json")
        .await
        .unwrap()
}

