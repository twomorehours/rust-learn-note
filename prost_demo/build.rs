use prost_build::Config;

fn main() {
    Config::new()
        .out_dir("src/pb")
        .bytes(&["."])
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["items.proto"], &["."])
        .unwrap();
}
