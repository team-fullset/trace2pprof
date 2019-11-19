fn main() {
    prost_build::compile_protos(&["src/profile.proto"], &["src/"]).unwrap();
}
