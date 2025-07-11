use prost_build::Config;

fn main() {
    Config::new()
        .type_attribute(
            ".",
            "#[derive(Deserialize,Serialize)] #[serde(rename_all = \"snake_case\")]",
        )
        .protoc_arg("--proto_path")
        .protoc_arg("./proto")
        .compile_protos(&["./proto/gtfs-realtime-NYCT.proto"], &["src/"])
        .unwrap();
}
