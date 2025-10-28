mod dispatch_impls;

use anyhow::{anyhow, Error};
use napi::{threadsafe_function::ThreadsafeFunction, Status};
use napi_derive::napi;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use wayland_client::{globals::registry_queue_init, protocol::wl_seat::WlSeat, Connection, Proxy};
use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notifier_v1::ExtIdleNotifierV1;

pub type UnitInfalliableCallback = ThreadsafeFunction<(), (), (), Status, false>;

#[derive(Default)]
struct IdleNotifierState {
    is_idle: Arc<Mutex<bool>>,
    on_idled: Arc<Mutex<Option<UnitInfalliableCallback>>>,
    on_resumed: Arc<Mutex<Option<UnitInfalliableCallback>>>,
}

#[napi(string_enum)]
pub enum IdleNotification {
    Idled,
    Resumed,
}

#[napi]
pub struct WaylandIdleNotifier {
    is_idle: Arc<Mutex<bool>>,
    on_idled: Arc<Mutex<Option<UnitInfalliableCallback>>>,
    on_resumed: Arc<Mutex<Option<UnitInfalliableCallback>>>,
}

#[napi]
impl WaylandIdleNotifier {
    #[napi(constructor)]
    pub fn new(timeout_ms: u32) -> Result<WaylandIdleNotifier, Error> {
        let is_idle = Arc::new(Mutex::new(false));
        let on_idled = Arc::new(Mutex::new(None));
        let on_resumed = Arc::new(Mutex::new(None));

        let state = IdleNotifierState {
            is_idle: is_idle.clone(),
            on_idled: on_idled.clone(),
            on_resumed: on_resumed.clone(),
        };

        watch_idle(state, timeout_ms)?;

        Ok(Self {
            is_idle,
            on_idled,
            on_resumed,
        })
    }

    #[napi]
    pub fn is_idle(&self) -> bool {
        *self.is_idle.lock().unwrap()
    }

    #[napi]
    pub fn on(
        &self,
        notification: IdleNotification,
        #[napi(ts_arg_type = "() => void")] callback: UnitInfalliableCallback,
    ) {
        match notification {
            IdleNotification::Idled => *self.on_idled.lock().unwrap() = Some(callback),
            IdleNotification::Resumed => *self.on_resumed.lock().unwrap() = Some(callback),
        }
    }
}

fn watch_idle(mut state: IdleNotifierState, timeout_ms: u32) -> Result<(), Error> {
    let connection = Connection::connect_to_env()?;
    let (globals, mut event_queue) = registry_queue_init(&connection)?;
    let queue_handle = event_queue.handle();

    let seats = globals.contents().clone_list();
    let interface_ver = seats
        .iter()
        .find(|g| g.interface == WlSeat::interface().name)
        .ok_or_else(|| anyhow!("no wl_seat found in global list"))?
        .version;
    let seat = globals.bind(&queue_handle, interface_ver..=interface_ver, ())?;

    globals
        .bind::<ExtIdleNotifierV1, IdleNotifierState, ()>(&queue_handle, 1..=1, ())?
        .get_idle_notification(timeout_ms, &seat, &queue_handle, ());

    thread::spawn(move || loop {
        if let Err(err) = event_queue.blocking_dispatch(&mut state) {
            eprintln!("{err}");
        };
    });

    Ok(())
}
