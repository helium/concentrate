extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["protos/gateway.proto"],
        includes: &["protos"],
        customize: protobuf_codegen_pure::Customize {
            expose_oneof: Some(true),
            expose_fields: Some(true),
            ..Default::default()
        },
    })
    .expect("protobuf codegen");
}
