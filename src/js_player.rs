use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread,
};

use futures::TryFutureExt;
use librespot::{
    connect::spirc::Spirc,
    playback::{
        mixer::Mixer,
        player::{Player, PlayerEvent, PlayerEventChannel},
    },
};
use neon::{
    prelude::{Channel, Context, FunctionContext, Handle, Object, Root},
    result::JsResult,
    types::{Deferred, Finalize, JsFunction, JsUndefined},
};
use tokio::runtime::Builder;

use crate::{
    player::{
        create_connect_config, create_credentials, create_player_config, create_session, new_player,
    },
    utils::create_js_obj_from_event,
};

impl Finalize for JsPlayerWrapper {}

pub struct JsPlayerWrapper {
    tx: mpsc::Sender<Message>,
}

pub type Callback = Box<dyn (FnOnce(&mut Spirc, &Channel, Deferred)) + Send>;

pub enum Message {
    Callback(Deferred, Callback),
    Close,
}

impl JsPlayerWrapper {
    pub fn new<'a, C>(cx: &mut C) -> Option<Self>
    where
        C: Context<'a>,
    {
        let (tx, rx) = mpsc::channel::<Message>();

        let (player_creation_tx, player_creation_rx) = mpsc::channel::<()>();

        let callback_channel = cx.channel();

        let channel = cx.channel();
        thread::spawn(move || {
            println!("Inside thread");
            let runtime = Builder::new_multi_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap();

            runtime.block_on(async {
                let credentials = create_credentials();
                let session = create_session().await;
                let player_config = create_player_config();
                let connect_config = create_connect_config();

                let (player, mixer) =
                    new_player(credentials.clone(), session.clone(), player_config.clone());

                let events_channel = player.get_player_event_channel();

                let (spirc, spirc_task) = Spirc::new(
                    connect_config.clone(),
                    session.clone(),
                    credentials.clone(),
                    player,
                    mixer,
                )
                .await
                .unwrap();

                JsPlayerWrapper::start_player_event_thread(channel, events_channel);
                JsPlayerWrapper::listen_commands(rx, spirc, callback_channel);

                // Panic thread if send fails
                player_creation_tx.send(()).unwrap();

                println!("watching spirc");
                spirc_task.await;
            })
        });

        while let Ok(_) = player_creation_rx.recv() {
            println!("Created player");
            return Some(Self { tx });
        }

        return None;
    }

    pub fn start_player_event_thread(channel: Channel, mut event_channel: PlayerEventChannel) {
        thread::spawn(move || loop {
            let message = event_channel.blocking_recv();
            if message.is_some() {
                channel.send(move |mut cx| {
                    let callback: Handle<JsFunction> = cx
                        .global()
                        .get(&mut cx, "_watch_player_events_global")
                        .unwrap();
                    let (obj, mut cx) = create_js_obj_from_event(cx, message.unwrap());
                    let _: JsResult<JsUndefined> =
                        callback.call_with(&mut cx).arg(obj).apply(&mut cx);
                    Ok(())
                });
            }
        });
    }

    pub fn listen_commands(rx: Receiver<Message>, mut spirc: Spirc, callback_channel: Channel) {
        thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                match message {
                    Message::Callback(deferred, f) => {
                        f(&mut spirc, &callback_channel, deferred);
                    }

                    Message::Close => break,
                }
            }
        });
    }

    pub fn close(&self) -> Result<(), mpsc::SendError<Message>> {
        self.tx.send(Message::Close)
    }

    pub fn send(
        &self,
        deferred: Deferred,
        callback: impl (FnOnce(&mut Spirc, &Channel, Deferred)) + Send + 'static,
    ) {
        let res = self
            .tx
            .send(Message::Callback(deferred, Box::new(callback)));
        if res.is_err() {
            panic!(
                "Failed to send command to player {}",
                res.err().unwrap().to_string()
            )
        }
    }

    pub fn set_player_events_listener(
        &self,
        callback: impl (FnOnce(PlayerEvent)) + Send + 'static,
    ) {
    }
}
