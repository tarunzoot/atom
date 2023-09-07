use super::AtomDownloadMetadata;
use crate::messages::MetadataMessage;
use iced::{subscription::unfold, Subscription};
use ring::digest::{Context, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

impl AtomDownloadMetadata {
    fn sha256_digest(path: &PathBuf) -> Result<String, bool> {
        if let Ok(input) = File::open(path) {
            let mut reader = BufReader::new(input);

            let mut context = Context::new(&SHA256);
            let mut buffer = [0; 1024];

            loop {
                if let Ok(count) = reader.read(&mut buffer) {
                    if count == 0 {
                        break;
                    }
                    context.update(&buffer[..count]);
                } else {
                    return Err(false);
                }
            }

            let mut digest = String::default();
            for c in context.finish().as_ref() {
                digest.push_str(&format!("{:02x}", c));
            }
            Ok(digest)
        } else {
            Err(false)
        }
    }

    pub fn subscription(&self) -> Subscription<MetadataMessage> {
        if !self.is_calculating_checksum {
            return Subscription::none();
        }

        unfold(
            self.url.len(),
            (self.file_path.clone(), self.url.clone()),
            move |(file_path, url)| async {
                if file_path.is_empty() {
                    iced::futures::future::pending().await
                } else {
                    let file_path = PathBuf::from(file_path);

                    if file_path.exists() {
                        if let Ok(digest) = AtomDownloadMetadata::sha256_digest(&file_path) {
                            return (
                                MetadataMessage::Checksum(digest, url),
                                (String::default(), String::default()),
                            );
                        }

                        (
                            MetadataMessage::Checksum(
                                "failed to calculate checksum!".to_owned(),
                                url,
                            ),
                            (String::default(), String::default()),
                        )
                    } else {
                        (
                            MetadataMessage::Checksum("error: file not found!".to_owned(), url),
                            (String::default(), String::default()),
                        )
                    }
                }
            },
        )
    }
}
