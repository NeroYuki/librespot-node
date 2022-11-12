use librespot::{core::spotify_id::SpotifyId, playback::player::PlayerEvent};
use neon::{
    prelude::{Context, Handle, Object},
    types::JsObject,
};

macro_rules! string {
    ($s: expr) => {
        String::from($s)
    };
}

pub fn create_js_obj_from_event<'a, C>(mut cx: C, event: PlayerEvent) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    match event {
        PlayerEvent::Stopped {
            play_request_id,
            track_id,
        } => default_to_obj(cx, string!("stopped"), play_request_id, track_id),
        PlayerEvent::Started {
            play_request_id,
            track_id,
            position_ms,
        } => started_to_obj(
            cx,
            string!("started"),
            play_request_id,
            track_id,
            position_ms,
        ),
        PlayerEvent::Changed {
            old_track_id,
            new_track_id,
        } => track_change_to_obj(cx, string!("changed"), old_track_id, new_track_id),
        PlayerEvent::Loading {
            play_request_id,
            track_id,
            position_ms,
        } => started_to_obj(
            cx,
            string!("loading"),
            play_request_id,
            track_id,
            position_ms,
        ),
        PlayerEvent::Preloading { track_id } => {
            default_to_obj(cx, string!("preloading"), 0, track_id)
        }
        PlayerEvent::Playing {
            play_request_id,
            track_id,
            position_ms,
            duration_ms,
        } => play_pause_to_obj(
            cx,
            string!("playing"),
            play_request_id,
            track_id,
            position_ms,
            duration_ms,
        ),
        PlayerEvent::Paused {
            play_request_id,
            track_id,
            position_ms,
            duration_ms,
        } => play_pause_to_obj(
            cx,
            string!("paused"),
            play_request_id,
            track_id,
            position_ms,
            duration_ms,
        ),
        PlayerEvent::TimeToPreloadNextTrack {
            play_request_id,
            track_id,
        } => default_to_obj(cx, string!("preload"), play_request_id, track_id),
        PlayerEvent::EndOfTrack {
            play_request_id,
            track_id,
        } => default_to_obj(cx, string!("ended"), play_request_id, track_id),
        PlayerEvent::Unavailable {
            play_request_id,
            track_id,
        } => default_to_obj(cx, string!("unavailable"), play_request_id, track_id),
        PlayerEvent::VolumeSet { volume } => volume_to_obj(cx, string!("volume"), volume),
    }
}

fn default_to_obj<'a, C>(
    mut cx: C,
    event: String,
    play_request_id: u64,
    track_id: SpotifyId,
) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    let obj = cx.empty_object();

    let ev = cx.string(event);
    obj.set(&mut cx, "event", ev).unwrap();

    let pri = cx.number(play_request_id as f64);
    obj.set(&mut cx, "play_request_id", pri).unwrap();

    let ti = cx.string(track_id.to_uri().unwrap_or("".to_string()));
    obj.set(&mut cx, "track_id", ti).unwrap();

    return (obj, cx);
}

fn volume_to_obj<'a, C>(mut cx: C, event: String, volume: u16) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    let obj = cx.empty_object();

    let ev = cx.string(event);
    obj.set(&mut cx, "event", ev).unwrap();

    let volume = cx.number(volume);
    obj.set(&mut cx, "volume", volume).unwrap();

    return (obj, cx);
}

fn track_change_to_obj<'a, C>(
    mut cx: C,
    event: String,
    old_track_id: SpotifyId,
    new_track_id: SpotifyId,
) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    let obj = cx.empty_object();

    let ev = cx.string(event);
    obj.set(&mut cx, "event", ev).unwrap();

    let oti = cx.string(old_track_id.to_uri().unwrap_or("".to_string()));
    obj.set(&mut cx, "old_track_id", oti).unwrap();

    let nti = cx.string(new_track_id.to_uri().unwrap_or("".to_string()));
    obj.set(&mut cx, "new_track_id", nti).unwrap();

    return (obj, cx);
}

fn started_to_obj<'a, C>(
    cx: C,
    event: String,
    play_request_id: u64,
    track_id: SpotifyId,
    position_ms: u32,
) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    let (obj, mut cx) = default_to_obj(cx, event, play_request_id, track_id);

    let pm = cx.number(position_ms);
    obj.set(&mut cx, "position_ms", pm).unwrap();

    return (obj, cx);
}

fn play_pause_to_obj<'a, C>(
    cx: C,
    event: String,
    play_request_id: u64,
    track_id: SpotifyId,
    position_ms: u32,
    duration_ms: u32,
) -> (Handle<'a, JsObject>, C)
where
    C: Context<'a>,
{
    let (obj, mut cx) = default_to_obj(cx, event, play_request_id, track_id);

    let pm = cx.number(position_ms);
    obj.set(&mut cx, "position_ms", pm).unwrap();

    let dm = cx.number(duration_ms);
    obj.set(&mut cx, "duration_ms", dm).unwrap();

    return (obj, cx);
}
