//! This build script is used to generate the code schema for the Seekstorm database, and save it to a json file that will be added in the executable.

/// Main function of the build
pub fn main() {
    use charybdis_parser::schema::code_schema::CodeSchema;
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/models/collection");
    let code_schema = CodeSchema::new(&"src/models/collection".to_string());
    let code_schema_json = serde_json::to_string(&code_schema).unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_file = std::path::Path::new(&out_dir).join("collection_code_schema.json");
    std::fs::write(out_file, code_schema_json).unwrap();
}
