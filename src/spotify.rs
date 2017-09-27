use cabot::{Client, RequestBuilder};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioFeatures {
    pub segments: Vec<Segment>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    pub start: f64,
    pub duration: f64,
    pub confidence: f64,
    pub loudness_start: f64,
    pub loudness_max_time: f64,
    pub loudness_max: f64,
    pub pitches: Vec<f32>,
    pub timbre: Vec<f32>,
}

pub fn get_audio_features(song_id: &str, bearer: &str) -> Option<AudioFeatures> {
    let request = RequestBuilder::new(&format!(
        "https://api.spotify.com/v1/audio-analysis/{}",
        song_id,
    )).add_header("Accept: application/json")
        .add_header(&format!("Authorization: Bearer {}", bearer))
        .build()
        .unwrap();
    let client = Client::new();
    client.execute(&request).ok().and_then(|response| {
        response
            .body_as_string()
            .ok()
            .and_then(|body| serde_json::from_str(&body).ok())
    })
}

pub fn get_audio_segments(song_id: &str, bearer: &str) -> Vec<Segment> {
    if let Some(audio_features) = get_audio_features(song_id, bearer) {
        return audio_features.segments;
    } else {
        Vec::new()
    }
}
