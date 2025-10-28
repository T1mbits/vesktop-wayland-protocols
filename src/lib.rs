mod dispatch_impls;

use anyhow::{anyhow, Error};
use napi_derive::napi;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use wayland_client::{globals::registry_queue_init, protocol::wl_seat::WlSeat, Connection, Proxy};
use wayland_protocols::ext::idle_notify::v1::client::ext_idle_notifier_v1::ExtIdleNotifierV1;

struct IdleState(Arc<Mutex<bool>>);

#[napi]
#[derive(Default)]
pub struct WaylandIdleWatcher {
    is_idle: Arc<Mutex<bool>>,
}

#[napi]
impl WaylandIdleWatcher {
    #[napi]
    pub fn new(timeout_ms: u32) -> Result<WaylandIdleWatcher, Error> {
        let is_idle = Arc::new(Mutex::new(false));
        watch_idle(is_idle.clone(), timeout_ms)?;
        Ok(Self { is_idle })
    }

    #[napi]
    pub fn is_idle(&self) -> bool {
        *self.is_idle.lock().expect("this should never be poisoned")
    }
}

fn watch_idle(is_idle: Arc<Mutex<bool>>, timeout_ms: u32) -> Result<(), Error> {
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
        .bind::<ExtIdleNotifierV1, IdleState, ()>(&queue_handle, 1..=1, ())?
        .get_idle_notification(timeout_ms, &seat, &queue_handle, ());

    let mut data = IdleState(is_idle);
    thread::spawn(move || loop {
        if let Err(err) = event_queue.blocking_dispatch(&mut data) {
            eprintln!("{err}");
        };
    });

    Ok(())
}
