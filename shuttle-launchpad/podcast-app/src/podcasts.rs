use std::io::BufReader;
use anyhow::Result;
use xml::EventReader;
use xml::reader::XmlEvent;

#[derive(Debug)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub audio_file: Option<String>,
}

impl Podcast {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            audio_file: None,
        }
    }
    pub fn to_html(&self) -> String {
        format!(
            r#"
            <html>
                <head>
                    <title>My Podcast: {}</title>
                </head>
                <body>
                    <h1>{}</h1>
                    <p>{}</p>
                    <audio controls src="{}"></audio>
                </body>
            </html>
        "#,
            self.title,
            self.title,
            self.description,
            match self.audio_file {
                Some(ref file) => file,
                None => "No audio available",
            }
        )
    }
}

#[derive(PartialEq)]
enum ParseState {
    Start,
    InItem,
    InTitle,
    InDescription,
}

pub async fn read_podcasts_from_xml(url: &str) -> Result<Vec<Podcast>> {
    let client = &reqwest::Client::new();
    let data = client.get(url)
        .header("Content-Type", "application/xml")
        .send().await?
        .text().await?;
    let parser = EventReader::new(BufReader::new(data.as_bytes()));
    let mut podcast = Podcast::new();
    let mut state = ParseState::Start;
    let mut results = Vec::new();
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                   name, attributes, ..
               }) => match name.local_name.as_str() {
                "item" => state = {
                    ParseState::InItem
                },
                "title" if state == ParseState::InItem && name.namespace.is_none() => {
                    state = ParseState::InTitle
                },
                "description" if state == ParseState::InItem && name.namespace.is_none() => state = ParseState::InDescription,
                "enclosure" if state == ParseState::InItem && name.namespace.is_none() => {
                    podcast.audio_file = attributes.into_iter().find_map(|attr| {
                        if attr.name.local_name == "url" {
                            Some(attr.value)
                        } else {
                            None
                        }
                    });
                }
                _ => {}
            },
            Ok(XmlEvent::CData(content)) | Ok(XmlEvent::Characters(content)) => match state {
                ParseState::InTitle => {
                    podcast.title = content;
                    state = ParseState::InItem;
                }
                ParseState::InDescription => {
                    podcast.description = content;
                    state = ParseState::InItem;
                }
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "item" {
                    results.push(podcast);
                    podcast = Podcast::new();
                    state = ParseState::InItem;
                }
            }
            _ => {}
        }
    }

    Ok(results)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_podcasts_from_url() {
        let url = "https://feeds.megaphone.fm/darknetdiaries";
        let podcasts = read_podcasts_from_xml(url).await.unwrap();

        let first_podcast = podcasts.last().unwrap();
        println!("{:?}", first_podcast);
        assert_eq!("Ep 1: The Phreaky World of PBX Hacking", first_podcast.title)
    }
}