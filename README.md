# vesktop-wayland-protocols

Simple wayland client implementation as a native node module for [Vesktop](https://github.com/Vencord/Vesktop).

Currently only provides a simple listener for [ext-idle-notify-v1](https://wayland.app/protocols/ext-idle-notify-v1), using [get_idle_notification](https://wayland.app/protocols/ext-idle-notify-v1#ext_idle_notifier_v1:request:get_idle_notification) for the idle notification object.

TODO:

- [ ] check if ext-idle-notify-v1 is supported by compositor before notifier init
- [ ] kill worker thread if notifier object is dropped and there are no callbacks (optional)
