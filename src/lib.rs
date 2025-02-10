#![feature(str_as_str)]
use std::{borrow::Cow, io::Write};

use chrono::{DateTime, TimeZone};
use quick_xml::{
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    Error, Writer,
};

#[derive(Debug, Clone, Default)]
pub struct AtomFeed<'a, Tz: TimeZone> {
    generator: Option<Generator<'a>>,
    published: Option<DateTime<Tz>>,
    updated: Option<DateTime<Tz>>,
    uri: Option<Cow<'a, str>>,
    self_uri: Option<Cow<'a, str>>,
    id: Option<Cow<'a, str>>,
    title: Cow<'a, str>,
    subtitle: Option<Cow<'a, str>>,
    rights: Option<Cow<'a, str>>,
    entries: Vec<AtomEntry<'a, Tz>>,
}

pub struct AtomFeedBuilder<'a, Tz: TimeZone>(AtomFeed<'a, Tz>);

impl<'a, Tz> AtomFeedBuilder<'a, Tz>
where
    Tz: TimeZone,
{
    pub fn new<T>(title: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self(AtomFeed {
            title: title.into(),
            generator: None,
            uri: None,
            self_uri: None,
            published: None,
            updated: None,
            id: None,
            subtitle: None,
            rights: None,
            entries: vec![],
        })
    }

    pub fn generator<T>(mut self, generator: Generator<'a>) -> Self {
        self.0.generator = Some(generator);
        self
    }

    pub fn uri<T>(mut self, uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.0.uri = Some(uri.into());
        self
    }

    pub fn self_uri<T>(mut self, uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.0.self_uri = Some(uri.into());
        self
    }

    pub fn id<T>(mut self, id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.0.id = Some(id.into());
        self
    }

    pub fn subtitle<T>(mut self, subtitle: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.0.subtitle = Some(subtitle.into());
        self
    }

    pub fn rights<T>(mut self, rights: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.0.rights = Some(rights.into());
        self
    }

    pub fn published<T>(mut self, published: DateTime<Tz>) -> Self {
        self.0.published = Some(published);
        self
    }

    pub fn updated<T>(mut self, updated: DateTime<Tz>) -> Self {
        self.0.updated = Some(updated);
        self
    }

    pub fn entries<T>(mut self, entries: Vec<AtomEntry<'a, Tz>>) -> Self {
        self.0.entries = entries;
        self
    }

    pub fn build(self) -> AtomFeed<'a, Tz> {
        self.0
    }
}

impl<'a, Tz> AtomFeed<'a, Tz>
where
    Tz: TimeZone,
{
    pub fn write_to<W: Write>(&self, writer: W) -> Result<W, Error> {
        let mut w = ::quick_xml::Writer::new(writer);
        self.write(&mut w)?;
        Ok(w.into_inner())
    }

    fn write<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))?;
        let mut tag = BytesStart::new("feed");
        tag.push_attribute(("xmlns", "http://www.w3.org/2005/Atom"));

        writer.write_event(Event::Start(tag))?;

        if let Some(generator) = &self.generator {
            generator.write(writer)?;
        }

        if let Some(self_uri) = &self.self_uri {
            let mut tag = BytesStart::new("link");
            tag.push_attribute(("href", self_uri.as_str()));
            tag.push_attribute(("rel", "self"));
            tag.push_attribute(("type", "application/atom+xml"));
            writer.write_event(Event::Empty(tag))?;
        }

        if let Some(uri) = &self.uri {
            let mut tag = BytesStart::new("link");
            tag.push_attribute(("href", uri.as_str()));
            tag.push_attribute(("rel", "alternate"));
            tag.push_attribute(("type", "text/html"));
            writer.write_event(Event::Empty(tag))?;
        }

        if let Some(published) = &self.published {
            writer
                .create_element("published")
                .write_text_content(BytesText::new(&published.to_rfc3339()))?;
        }

        if let Some(updated) = &self.updated {
            writer
                .create_element("updated")
                .write_text_content(BytesText::new(&updated.to_rfc3339()))?;
        }

        if let Some(id) = &self.id {
            writer
                .create_element("id")
                .write_text_content(BytesText::new(id))?;
        }

        writer
            .create_element("title")
            .write_text_content(BytesText::new(&self.title))?;

        if let Some(subtitle) = &self.subtitle {
            writer
                .create_element("subtitle")
                .write_text_content(BytesText::new(subtitle))?;
        }

        for entry in &self.entries {
            entry.write(writer)?;
        }

        writer.write_event(Event::End(BytesEnd::new("feed")))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Generator<'a> {
    uri: Option<Cow<'a, str>>,
    version: Option<Cow<'a, str>>,
    name: Cow<'a, str>,
}

impl<'a> Generator<'a> {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            uri: None,
            version: None,
        }
    }

    pub fn uri<T>(mut self, uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.uri = Some(uri.into());
        self
    }

    pub fn version<T>(mut self, version: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.version = Some(version.into());
        self
    }

    fn write<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
        let mut tag = BytesStart::new("generator");

        if let Some(uri) = &self.uri {
            tag.push_attribute(("uri", uri.as_str()));
        }

        if let Some(version) = &self.version {
            tag.push_attribute(("version", version.as_str()));
        }

        writer.write_event(Event::Start(tag))?;
        writer.write_event(Event::Text(BytesText::new(&self.name)))?;
        writer.write_event(Event::End(BytesEnd::new("generator")))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Person<'a> {
    name: Cow<'a, str>,
    uri: Option<Cow<'a, str>>,
    email: Option<Cow<'a, str>>,
}

impl<'a> Person<'a> {
    pub fn new<T>(name: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            uri: None,
            email: None,
        }
    }

    pub fn uri<T>(mut self, uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.uri = Some(uri.into());
        self
    }

    pub fn email<T>(mut self, email: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.email = Some(email.into());
        self
    }

    fn write<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
        writer
            .create_element("name")
            .write_text_content(BytesText::new(&self.name))?;

        if let Some(uri) = &self.uri {
            writer
                .create_element("uri")
                .write_text_content(BytesText::new(uri))?;
        }

        if let Some(email) = &self.email {
            writer
                .create_element("email")
                .write_text_content(BytesText::new(email))?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct AtomEntry<'a, Tz: TimeZone> {
    title: Cow<'a, str>,
    uri: Option<Cow<'a, str>>,
    published: Option<DateTime<Tz>>,
    updated: Option<DateTime<Tz>>,
    id: Option<Cow<'a, str>>,
    categories: Vec<Cow<'a, str>>,
    authors: Vec<Person<'a>>,
    contributors: Vec<Person<'a>>,
    content: Option<Cow<'a, str>>,
    summary: Option<Cow<'a, str>>,
}

impl<'a, Tz> AtomEntry<'a, Tz>
where
    Tz: TimeZone,
{
    pub fn new<T>(title: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            title: title.into(),
            uri: None,
            published: None,
            updated: None,
            id: None,
            categories: vec![],
            authors: vec![],
            contributors: vec![],
            content: None,
            summary: None,
        }
    }

    pub fn uri<T>(mut self, uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.uri = Some(uri.into());
        self
    }

    pub fn id<T>(mut self, id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.id = Some(id.into());
        self
    }

    pub fn published<T>(mut self, published: DateTime<Tz>) -> Self {
        self.published = Some(published);
        self
    }

    pub fn updated<T>(mut self, updated: DateTime<Tz>) -> Self {
        self.updated = Some(updated);
        self
    }

    pub fn categories<T>(mut self, categories: Vec<Cow<'a, str>>) -> Self {
        self.categories = categories;
        self
    }

    pub fn authors<T>(mut self, authors: Vec<Person<'a>>) -> Self {
        self.authors = authors;
        self
    }

    pub fn contributors<T>(mut self, contributors: Vec<Person<'a>>) -> Self {
        self.contributors = contributors;
        self
    }

    pub fn content<T>(mut self, content: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.content = Some(content.into());
        self
    }

    pub fn summary<T>(mut self, summary: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.summary = Some(summary.into());
        self
    }

    fn write<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), Error> {
        writer.write_event(Event::Start(BytesStart::new("entry")))?;

        writer
            .create_element("title")
            .write_text_content(BytesText::new(&self.title))?;

        if let Some(uri) = &self.uri {
            let mut element = BytesStart::new("link");
            element.push_attribute(("href", uri.as_str()));
            element.push_attribute(("rel", "alternate"));
            element.push_attribute(("type", "text/html"));
            element.push_attribute(("title", self.title.as_str()));
            writer.write_event(Event::Empty(element))?;
        }

        if let Some(published) = &self.published {
            writer
                .create_element("published")
                .write_text_content(BytesText::new(&published.to_rfc3339()))?;
        }

        if let Some(updated) = &self.updated {
            writer
                .create_element("updated")
                .write_text_content(BytesText::new(&updated.to_rfc3339()))?;
        }

        if let Some(id) = &self.id {
            writer
                .create_element("id")
                .write_text_content(BytesText::new(id))?;
        }

        for author in &self.authors {
            writer.write_event(Event::Start(BytesStart::new("author")))?;

            author.write(writer)?;

            writer.write_event(Event::End(BytesEnd::new("author")))?;
        }

        for contributor in &self.contributors {
            writer.write_event(Event::Start(BytesStart::new("contributor")))?;

            contributor.write(writer)?;

            writer.write_event(Event::End(BytesEnd::new("contributor")))?;
        }

        for category in &self.categories {
            let mut tag = BytesStart::new("category");
            tag.push_attribute(("term", category.as_str()));
            writer.write_event(Event::Empty(tag))?;
        }

        if let Some(summary) = &self.summary {
            writer
                .create_element("summary")
                .with_attribute(("type", "html"))
                .write_text_content(BytesText::new(summary))?;
        }

        if let Some(content) = &self.content {
            writer
                .create_element("content")
                .with_attribute(("type", "html"))
                .write_text_content(BytesText::new(content))?;
        }

        writer.write_event(Event::End(BytesEnd::new("entry")))?;
        Ok(())
    }
}
