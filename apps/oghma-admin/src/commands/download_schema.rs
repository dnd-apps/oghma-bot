use log::info;
use oghma_graphql::introspection;
use std::fs::write;
use std::path::Path;

pub async fn download_schema(out_path: Option<String>) {
    let introspection = introspection();

    if let Some(out) = out_path {
        info!("Writing introspection to {out}");
        write(Path::new(&out), introspection).expect("Failed to write introspection!");
        info!("Write successful! you can find the introspection at: {out}");
    } else {
        info!("\n\n{introspection}");
    }
}
