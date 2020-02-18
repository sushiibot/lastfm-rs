use crate::error::{Error, LastFMError};
use crate::user::User;
use crate::{Client, RequestBuilder};
use serde::Deserialize;
use std::io::Read;
use std::marker::PhantomData;

/// The main top artists structure.
///
/// This is splitted off into two areas: One, the attributes (used
/// for displaying various user-associated attributes), and two,
/// the user's top artists.
///
/// For details on the attributes available, refer to [Attributes]. For
/// details on the artist information available, refer to [Artist].
#[derive(Debug, Deserialize)]
pub struct TopArtists {
    #[serde(rename = "artist")]
    pub artists: Vec<Artist>,
    #[serde(rename = "@attr")]
    pub attrs: Attributes,
}

#[derive(Debug, Deserialize)]
pub struct Attributes {
    pub page: String,
    pub total: String,
    pub user: String,
    #[serde(rename = "perPage")]
    pub per_page: String,
    #[serde(rename = "totalPages")]
    pub total_pages: String,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    #[serde(rename = "@attr")]
    pub attrs: ArtistAttributes,
    pub mbid: String,
    pub playcount: String,
    pub name: String,
    pub url: String,
    #[serde(rename = "image")]
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistAttributes {
    pub rank: String,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    #[serde(rename = "size")]
    pub image_size: String,
    #[serde(rename = "#text")]
    pub image_url: String,
}

impl TopArtists {
    pub fn build<'a>(client: &'a mut Client, user: &str) -> RequestBuilder<'a, TopArtists> {
        let url = client.build_url(vec![("method", "user.getTopArtists"), ("user", user)]);

        RequestBuilder { client, url, phantom: PhantomData }
    }
}

/// Allows users to specify the period of which they'd like to retrieve
/// top artist data for.
pub enum Period {
    Overall,
    SevenDays,
    OneMonth,
    ThreeMonths,
    SixMonths,
    TwelveMonths,
    OneYear,
}

impl ToString for Period {
    /// Converts the given period to a string. In most cases,
    /// you won't have to use this yourself. Period durations
    /// will usually be automatically converted to their string
    /// form when fed to the `with_period` parameter function.
    fn to_string(&self) -> String {
        match self {
            Self::Overall => String::from("overall"),
            Self::SevenDays => String::from("7day"),
            Self::OneMonth => String::from("1month"),
            Self::ThreeMonths => String::from("3month"),
            Self::SixMonths => String::from("6month"),
            // TwelveMonths and OneYear are exactly the same,
            // it just allows users to choose which one they'd
            // like to use, depending on verbosity / simplicity.
            Self::TwelveMonths => String::from("12month"),
            Self::OneYear => String::from("12month"),
        }
    }
}

impl<'a> RequestBuilder<'a, TopArtists> {
    add_param!(with_limit, limit, usize);
    add_param!(with_period, period, Period);
    add_param!(with_page, page, usize);

    pub fn send(&'a mut self) -> Result<TopArtists, Error> {
        match self.client.request(&self.url) {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                match serde_json::from_str::<LastFMError>(&*body) {
                    Ok(lastm_error) => Err(Error::LastFMError(lastm_error.into())),
                    Err(_) => match serde_json::from_str::<User>(&*body) {
                        Ok(user) => Ok(user.top_artists.unwrap()),
                        Err(e) => Err(Error::ParsingError(e)),
                    },
                }
            }
            Err(err) => Err(Error::HTTPError(err)),
        }
    }
}

impl<'a> Client {
    pub fn top_artists(&'a mut self, user: &str) -> RequestBuilder<'a, TopArtists> {
        TopArtists::build(self, user)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::make_client;
    use crate::user::top_artists::Period::*;

    #[test]
    fn test_top_artists() {
        let mut client = make_client();
        let top_artists = client.top_artists("LAST.HQ").with_limit(1).send();
        println!("{:#?}", top_artists);
        assert!(top_artists.is_ok());
    }

    #[test]
    fn test_top_artists_overall() {
        let mut client = make_client();
        let top_artists_overall = client.top_artists("LAST.HQ").with_period(Overall).with_limit(5).send();
        println!("{:#?}", top_artists_overall);
        assert!(top_artists_overall.is_ok());
    }

    #[test]
    fn test_top_artists_7_days() {
        let mut client = make_client();
        let top_artists_7_days = client.top_artists("LAST.HQ").with_period(SevenDays).with_limit(5).send();
        println!("{:#?}", top_artists_7_days);
        assert!(top_artists_7_days.is_ok());
    }

    #[test]
    fn test_top_artists_1_month() {
        let mut client = make_client();
        let top_artists_1_month = client.top_artists("LAST.HQ").with_period(OneMonth).with_limit(5).send();
        println!("{:#?}", top_artists_1_month);
        assert!(top_artists_1_month.is_ok());
    }

    #[test]
    fn test_top_artists_3_months() {
        let mut client = make_client();
        let top_artists_3_months = client.top_artists("LAST.HQ").with_period(ThreeMonths).with_limit(5).send();
        println!("{:#?}", top_artists_3_months);
        assert!(top_artists_3_months.is_ok());
    }

    #[test]
    fn test_top_artists_6_months() {
        let mut client = make_client();
        let top_artists_6_months = client.top_artists("LAST.HQ").with_period(SixMonths).with_limit(5).send();
        println!("{:#?}", top_artists_6_months);
        assert!(top_artists_6_months.is_ok());
    }

    #[test]
    fn test_top_artists_12_months() {
        let mut client = make_client();
        let top_artists_12_months = client.top_artists("LAST.HQ").with_period(OneYear).with_limit(5).send();
        println!("{:#?}", top_artists_12_months);
        assert!(top_artists_12_months.is_ok());
    }
}