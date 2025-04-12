fn main() {
    // For tests7: Set the environment variable TEST_FOO with a timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 获取当前时间戳（从 Unix 纪元开始的秒数）
    println!("cargo:rustc-env=TEST_FOO={}", timestamp); // 设置环境变量 TEST_FOO

    // For tests8: Enable the "pass" feature to make the test return early
    println!("cargo:rustc-cfg=pass"); 
}