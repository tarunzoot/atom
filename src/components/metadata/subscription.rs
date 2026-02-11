use super::AtomDownloadMetadata;
use crate::messages::MetadataMessage;
use iced::{
    futures::stream::{self, Stream},
    Subscription,
};
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

    fn sha256_subscription(
        state: &(
            String, // file_path
            String, // url
        ),
    ) -> impl Stream<Item = MetadataMessage> {
        let (file_path, url) = (state.0.clone(), state.1.clone());

        stream::unfold((file_path, url), |(file_path, url)| async move {
            if file_path.is_empty() {
                iced::futures::future::pending().await
            }

            let path = PathBuf::from(&file_path);

            let message = if path.exists() {
                match AtomDownloadMetadata::sha256_digest(&path) {
                    Ok(digest) => MetadataMessage::Checksum(digest, url),
                    Err(_) => {
                        MetadataMessage::Checksum("failed to calculate checksum!".to_owned(), url)
                    }
                }
            } else {
                MetadataMessage::Checksum("error: file not found!".to_owned(), url)
            };

            Some((message, (String::new(), String::new())))
        })
    }

    pub fn subscription(&self) -> Subscription<MetadataMessage> {
        if !self.is_calculating_checksum {
            return Subscription::none();
        }

        Subscription::run_with(
            (self.file_path.clone(), self.url.clone()),
            AtomDownloadMetadata::sha256_subscription,
        )
    }
}
