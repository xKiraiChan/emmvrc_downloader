use std::io::Write;

#[tokio::main]
async fn main() {
    std::fs::File::create("./emmVRC.dll").unwrap()
    .write_all(
        &base64::decode(
            &reqwest::get("https://thetrueyoshifan.com/BakaUpdate.php?libdownload")
                .await
                .expect("Downloading")
                .bytes()
                .await
                .unwrap()
        ).unwrap()
    ).expect("Saving");
}
