use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, error::PutObjectError, types::SdkError};
use std::fs;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use thiserror::Error;
use tokio::io::AsyncReadExt;

#[derive(Error, Debug)]
enum UploadError {
  #[error("S3 error: {0}")]
  S3(#[from] aws_sdk_s3::Error),
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),
  #[error("S3 put object error: {0}")]
  PutObject(#[from] SdkError<PutObjectError>),
}

#[tokio::main]
async fn main() -> Result<(), UploadError> {
  let bucket_name = "your bucket-name"; // Your specified bucket name
  let path = "/path/to/file/or/directory"; // Your specified path

  let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
  let config = aws_config::from_env().region(region_provider).load().await;
  let client = Client::new(&config);

  let path = Path::new(path);
  if path.is_file() {
    upload_file(&client, bucket_name, path).await?;
  } else if path.is_dir() {
    upload_directory(&client, bucket_name, path).await.await?;
  } else {
    println!("Error: Path does not exist or is not a file/directory");
  }

  Ok(())
}

async fn upload_file(
  client: &Client,
  bucket_name: &str,
  file_path: &Path,
) -> Result<(), UploadError> {
  let file_name = file_path
    .file_name()
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid file name"))?
    .to_str()
    .ok_or_else(|| {
      std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid file name encoding",
      )
    })?;
  let mut file = tokio::fs::File::open(file_path).await?;
  let mut contents = Vec::new();
  file.read_to_end(&mut contents).await?;

  client
    .put_object()
    .bucket(bucket_name)
    .key(file_name)
    .body(contents.into())
    .send()
    .await?;

  println!("Uploaded file: {}", file_name);
  Ok(())
}

async fn upload_directory<'a>(
  client: &'a Client,
  bucket_name: &'a str,
  dir_path: &'a Path,
) -> Pin<Box<dyn Future<Output = Result<(), UploadError>> + 'a>> {
  Box::pin(async move {
    let paths = fs::read_dir(dir_path)?;
    for path in paths {
      let path = path?.path();
      if path.is_file() {
        upload_file(client, bucket_name, &path).await?;
      } else if path.is_dir() {
        upload_directory(client, bucket_name, &path).await.await?;
      }
    }
    Ok(())
  })
}
