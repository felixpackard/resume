use std::{env, fs, path::Path};

use build_print::println;
use typify::{TypeSpace, TypeSpaceImpl, TypeSpaceSettings};

fn main() {
    let content = std::fs::read_to_string("schema.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(false)
            .with_replacement(
                "Iso8601",
                "json::FlexibleIsoDate",
                [TypeSpaceImpl::Display].into_iter(),
            ),
    );
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new(&env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");

    println!("Writing typify output to {}", out_file.to_string_lossy());
    fs::write(out_file, contents).unwrap();
}
