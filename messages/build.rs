extern crate protobuf_codegen_pure;

fn main() {
    println!("cargo:rerun-if-changed=protos/gateway.proto");

    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["proto/src/longfi.proto", "proto/src/radio.proto"],
        includes: &["proto"],
        customize: protobuf_codegen_pure::Customize {
            expose_oneof: Some(true),
            expose_fields: Some(true),
            ..Default::default()
        },
    })
    .expect("protobuf codegen");
}
