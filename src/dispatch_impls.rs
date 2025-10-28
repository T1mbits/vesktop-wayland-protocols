use crate::IdleNotifierState;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use wayland_client::{
    globals::GlobalListContents,
    protocol::{wl_registry::WlRegistry, wl_seat::WlSeat},
    Connection, Dispatch, Proxy, QueueHandle,
};
use wayland_protocols::ext::idle_notify::v1::client::{
    ext_idle_notification_v1::{Event, ExtIdleNotificationV1},
    ext_idle_notifier_v1::ExtIdleNotifierV1,
};

impl Dispatch<WlRegistry, GlobalListContents> for IdleNotifierState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: <WlRegistry as Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSeat, ()> for IdleNotifierState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSeat,
        _event: <WlSeat as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtIdleNotifierV1, ()> for IdleNotifierState {
    fn event(
        _state: &mut Self,
        _proxy: &ExtIdleNotifierV1,
        _event: <ExtIdleNotifierV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtIdleNotificationV1, ()> for IdleNotifierState {
    fn event(
        state: &mut Self,
        _proxy: &ExtIdleNotificationV1,
        event: <ExtIdleNotificationV1 as Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        let (is_idle, callback) = match event {
            Event::Idled => (true, state.on_idled.lock().unwrap()),
            Event::Resumed => (false, state.on_resumed.lock().unwrap()),
            _ => unreachable!("notification events only have idled and resumed events as of v1"),
        };

        *state.is_idle.lock().unwrap() = is_idle;

        if let Some(callback) = callback.as_ref() {
            callback.call((), ThreadsafeFunctionCallMode::Blocking);
        }
    }
}
