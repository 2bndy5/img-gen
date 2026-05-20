use std::{env, error::Error, fs, path::Path};

use fontsource_downloader::{FontQuery, FontSourceClient};

const ROBOTO_PATH: &str = "assets/fonts/roboto-latin-400-normal.ttf";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("cargo:rerun-if-changed={ROBOTO_PATH}");
    println!("cargo:rerun-if-env-changed=IMG_GEN_REFRESH_DEFAULT_FONT");

    if let Err(error) = sync_default_font().await {
        panic!("failed to prepare embedded default font: {error}");
    }
}

async fn sync_default_font() -> Result<(), Box<dyn Error>> {
    let font_path = Path::new(ROBOTO_PATH);
    let refresh = env::var("IMG_GEN_REFRESH_DEFAULT_FONT").is_ok_and(|value| value == "1");

    if font_path.exists() && !refresh {
        return Ok(());
    }

    if let Some(parent) = font_path.parent() {
        fs::create_dir_all(parent)?;
    }

    match download_font().await {
        Ok(bytes) => {
            fs::write(font_path, &bytes)?;
            Ok(())
        }
        Err(error) if font_path.exists() => {
            println!(
                "cargo:warning=Could not refresh default font from fontsource: {error}. Using existing file."
            );
            Ok(())
        }
        Err(error) => Err(error),
    }
}

async fn download_font() -> Result<Vec<u8>, Box<dyn Error>> {
    let query = FontQuery {
        family: "Roboto".to_string(),
        ..Default::default()
    };
    let fontsource_client = FontSourceClient::new()?;
    let font_path = fontsource_client.download_font(&query).await?;
    Ok(fs::read(&font_path)?)
}
