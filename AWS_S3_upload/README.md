# Rust AWS S3 Uploader Documentation

## Overview

This Rust program uploads either a single file or all files within a directory to an Amazon S3 bucket. It uses the **AWS SDK for Rust** and is fully asynchronous with **Tokio**.

---

## Features

- Upload a single file to a specified S3 bucket.
- Recursively upload all files within a specified directory.
- Automatically detects the AWS region (falls back to `us-east-1` if not found).
- Structured error handling using `thiserror`.

---

## Dependencies

Make sure the following crates are included in your `Cargo.toml`:

```toml
[dependencies]
aws-config = "1.1.1"
aws-sdk-s3 = "1.15.0"
tokio = { version = "1", features = ["full"] }
thiserror = "1.0"
```

---

## Usage

1. Update the following constants in `main()`:

   - `bucket_name`: Name of your target S3 bucket.
   - `path`: Absolute path to a file or directory you want to upload.

2. Run the program:

```bash
cargo run
```

---

## Code Structure

### `main()`

- Loads AWS configuration with a fallback region.
- Initializes the S3 client.
- Checks if the input path is a file or directory.

  - Calls `upload_file` or `upload_directory` accordingly.

---

### `upload_file()`

Uploads a single file to the specified S3 bucket.

**Parameters:**

- `client`: Reference to the AWS S3 client.
- `bucket_name`: Target S3 bucket name.
- `file_path`: Path to the file to upload.

**Steps:**

- Reads the file contents asynchronously.
- Uploads it to S3 using `put_object`.
- Uses the file name as the object key.

---

### `upload_directory()`

Recursively traverses a directory and uploads all files inside it.

**Parameters:**

- `client`: AWS S3 client.
- `bucket_name`: S3 bucket name.
- `dir_path`: Directory path.

**Steps:**

- Iterates through the directory entries.
- Calls `upload_file()` on files.
- Recursively calls `upload_directory()` on subdirectories.

---

## Error Handling

Errors are captured using a custom enum `UploadError` that implements `thiserror::Error`.

Possible errors:

- S3 errors from the AWS SDK.
- I/O errors when reading files.
- `PutObjectError` from failed uploads.

---

## Notes

- Subdirectory structure is **not preserved** in the S3 keys — only file names are used.
- If you need to retain folder structure in the bucket, additional logic is required to construct the relative path key.

---

## Example Output

```
Uploaded file: myfile.txt
Uploaded file: image.png
```
