pub type Recipes = Response<Recipe>;

pub struct Recipe {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub instructions: Vec<String>,
    pub body: String,
    pub ingredients: Vec<Ingredient>,
    pub url: String,
}



