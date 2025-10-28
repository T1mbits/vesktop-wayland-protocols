const { IdleNotifier } = require(".");

const idle_notifier = new IdleNotifier({
    timeoutMs: 1000,
    onIdled: () => console.log("event: idled"),
    onResumed: () => console.log("event: resumed")
});

function tick() {
    console.log("tick: ", idle_notifier.isIdle());
    setTimeout(tick, 1000);
}

tick();
