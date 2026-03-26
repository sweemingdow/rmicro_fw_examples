fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos = Vec::new();

    // 1. 收集需要编译的文件，并告诉 Cargo 监听它们
    if cfg!(feature = "user_api") {
        let p = "../proto/user_api.proto";
        protos.push(p);
        println!("cargo:rerun-if-changed={}", p);
    }

    if cfg!(feature = "order_api") {
        let p = "../proto/order_api.proto";
        protos.push(p);
        println!("cargo:rerun-if-changed={}", p);
    }

    // 2. 只要有任何一个 feature 开启，就执行编译
    if !protos.is_empty() {
        tonic_build::configure()
            // 注意：不建议 out_dir("src")，
            // 默认生成的代码在 OUT_DIR 目录下，这符合 include_proto! 的预期
            .build_server(true)
            .build_client(true)
            .compile_protos(
                &protos,
                &["../proto"],
            )?;
    }

    Ok(())
}