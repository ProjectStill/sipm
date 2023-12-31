use super::sources;
use std::{any::Any, fs, io::Read};
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

pub use super::conf::Config;

pub fn get_metadata_and_source(file_path: &str) -> (RecipeMetadata, Yaml) {
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let recipe = YamlLoader::load_from_str(&contents).unwrap();
    let metadata = &recipe[0]["metadata"];

    let categories_vec = metadata["categories"].as_vec().unwrap();
    let categories = categories_vec
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect(); // Convert categories to Vec<String> from YAML

    let recipe_metadata = RecipeMetadata {
        name: metadata["name"].as_str().unwrap().to_string(),
        description: metadata["description"].as_str().unwrap().to_string(),
        author: metadata["author"].as_str().unwrap().to_string(),
        package_author: metadata["package_author"].as_str().unwrap().to_string(),
        package_version: metadata["package_version"].as_f64().unwrap(),
        license: metadata["license"].as_str().unwrap().to_string(),
        url: metadata["url"].as_str().unwrap().to_string(),
        categories,
        source_type: metadata["type"].as_str().unwrap().to_string(),
        can_auto_update: metadata["can_auto_update"].as_bool().unwrap(),
        arch: metadata["arch"].as_str().unwrap().to_string(),
    };

    let source_data = &recipe[0][recipe_metadata.source_type.as_str()];
    (recipe_metadata, source_data.clone())
}

#[derive(Debug)]
pub struct Recipe {
    metadata: RecipeMetadata,
    source_data: Box<dyn Any>,
    source_type: dyn Any,
}
pub trait RecipeFunctions {
    fn check_metadata(&self, metadata: RecipeMetadata) -> bool {
        // Check if the metadata is valid
        sources::SOURCE_LIST.contains(&metadata.source_type.as_str())
    }
    fn check_source_data(&self, source_data: dyn Any) -> bool;
    fn parse_yaml(&self, file: &str) -> Recipe;
}

#[derive(Debug)]
pub struct RecipeMetadata {
    name: String,
    description: String,
    author: String,
    package_author: String,
    package_version: f64,
    license: String,
    url: String,
    categories: Vec<String>,
    source_type: String,
    can_auto_update: bool,
    arch: String,
}

pub trait Source {
    const NAME: String;
    const CUSTOM_FUNCTIONS: Vec<String> = vec![];
    const SEPARATE_FROM_MULTIPM_REPO: bool = false;
    const SUPPORTED_LOCAL_FILE_TYPES: Vec<String> = vec![];

    fn initialize(&self, _: Config) -> i32 {
        0
    }
    fn install(&self, _: Recipe, _: Config) -> i32;
    fn install_local_file(&self, _: Recipe, _: Config) -> i32;
    fn install_with_files(&self, _: Recipe, _: Config) -> i32;
    fn remove(&self, _: Recipe, _: Config) -> i32;
    fn update_all(&self, _: Recipe, _: Config) -> i32;
    fn update_pkg(&self, _: Recipe, _: Config) -> i32;
    fn update_with_files(&self, _: Recipe, _: Config) -> i32;
    fn validate_recipe(&self, _: Recipe) -> i32;
    fn generate_recipe(&self, metadata: RecipeMetadata, source_data: dyn Any) -> (i32, Recipe);
    fn interactive_recipe_generator(&self) -> (i32, dyn Any);
    fn sync_repo_cache(&self, _: Config) -> i32 {
        0
    }
}

pub struct DependentFile {
    file_glob: String,
    url: String,
}
