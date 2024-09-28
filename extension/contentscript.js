window.addEventListener('keydown', function (key) {
    let keyvalue = key.key.toLowerCase();
    if (keyvalue == 'alt') {
        let message = { key: keyvalue, state: 'down' };
        chrome.runtime.sendMessage(null, message, (response) => {});
    }
});

window.addEventListener('keyup', function (key) {
    let keyvalue = key.key.toLowerCase();
    if (keyvalue == 'alt') {
        let message = { key: keyvalue, state: 'up' };
        chrome.runtime.sendMessage(null, message, (response) => {});
    }
});
