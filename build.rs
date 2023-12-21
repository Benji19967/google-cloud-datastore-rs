fn main() {
    tonic_build::configure()
        .build_server(false)
        //.out_dir("src/google")  // you can change the generated code's location
        .compile(
            &["proto/googleapis/google/datastore/v1/datastore.proto"],
            &["proto/googleapis"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
