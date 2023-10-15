use aral::fs::File;

pub fn test() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            create_file().await;
        });
}

pub async fn create_file() {
    struct MyFile<F: File>(F);

    let f = aral::fs::create_file("/tmp/test-file.txt").await.unwrap();
    let mf = MyFile(f);
    mf.0.sync_all();
}
