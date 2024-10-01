use serde::{Deserialize, Serialize};

use crate::models::Meetup;

/// Represents the options that can be used to filter a list of items.
#[derive(Debug, Deserialize, Default)]
pub struct FilterOptions {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMeetup {
    pub meetup_id: i64,
    pub title: String,
    pub body_text: String,
}

impl From<Meetup> for GetMeetup {
    fn from(meetup: Meetup) -> Self {
        Self {
            meetup_id: meetup.meetup_id,
            title: meetup.title,
            body_text: meetup.body_text,
        }
    }
}

impl<'a> From<&'a Meetup> for GetMeetup {
    fn from(meetup: &'a Meetup) -> Self {
        Self {
            meetup_id: meetup.meetup_id,
            title: meetup.title.clone(),
            body_text: meetup.body_text.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateMeetup {
    pub title: String,
    pub body_text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMeetup {
    pub title: Option<String>,
    pub body_text: Option<String>,
}