use std::process::Command;

fn main() {
    tonic_prost_build::configure()
        .build_server(false)
        .out_dir("src/pb")
        .compile_protos(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    Command::new("cargo").args(&["fmt"]).output().unwrap();
}
