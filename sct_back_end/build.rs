use indexmap::IndexMap;
use openapiv3::{OpenAPI, Paths};
use progenitor::Generator;

fn generate_types_only(source_spec_path: &str, output_directory: &str) {
  // Read and parse OpenAPI spec
  let yaml = std::fs::read_to_string(source_spec_path).expect("Failed to read OpenAPI spec");
  let open_api: OpenAPI = serde_yml::from_str(&yaml).expect("Failed to parse OpenAPI spec");

  // Drop paths, since we are not interested in them.
  // Not really necessary, because we drop non-type code later, too,
  // but still a sensible approach here.
  let other_spec = OpenAPI {
    paths: Paths {
      paths: IndexMap::new(),
      extensions: IndexMap::new(),
    },
    ..open_api
  };

  // Use the correct method on Generator
  let mut generator = Generator::default();
  let tokens = generator
    .generate_tokens(&other_spec)
    .expect("Failed to generate tokens from OpenAPI spec");

  let ast = syn::parse2::<syn::File>(tokens).unwrap();

  // Extract only the types module
  let types_module = ast
    .items
    .iter()
    .find_map(|item| {
      if let syn::Item::Mod(module) = item {
        if module.ident == "types" {
          return Some(module.clone());
        }
      }
      None
    })
    .expect("Could not find types module in generated code");

  // Create a new AST with only the types module
  let new_ast = syn::File {
    shebang: None,
    attrs: vec![],
    items: vec![syn::Item::Mod(types_module)],
  };

  let content = prettyplease::unparse(&new_ast);

  let out_file = std::path::Path::new(output_directory).join("generated.rs");

  std::fs::write(out_file, content).unwrap();
}

fn main() {
  let sct_spec_path = "../open_api/sct.yaml";
  println!("cargo:rerun-if-changed={}", sct_spec_path);

  generate_types_only(sct_spec_path, "src/models/sct");

  // Todo: Add Steam facing OpenAPI spec generation, too
}
