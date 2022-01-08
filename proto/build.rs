fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["IDL/route_guide.proto"], &["IDL"])
        .unwrap();
}
