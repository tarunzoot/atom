window.addEventListener('keydown', function (key) {
    if (!key.key) return;
    let keyvalue = key.key.toLowerCase();

    if (keyvalue == 'alt') {
        let message = { key: keyvalue, state: 'down' };
        chrome.runtime.sendMessage(null, message, (response) => {});
    }
});

window.addEventListener('keyup', function (key) {
    if (!key.key) return;
    let keyvalue = key.key.toLowerCase();
    if (keyvalue == 'alt') {
        let message = { key: keyvalue, state: 'up' };
        chrome.runtime.sendMessage(null, message, (response) => {});
    }
});
