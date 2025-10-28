const { WaylandIdleNotifier, IdleNotification } = require(".");

const idle_notifier = new WaylandIdleNotifier(1000);
idle_notifier.on(IdleNotification.Idled, () => console.log("event: idled"));
idle_notifier.on(IdleNotification.Resumed, () => console.log("event: resumed"));

function tick() {
    console.log("tick: ", idle_notifier.isIdle());
    setTimeout(tick, 1000);
}

tick();
