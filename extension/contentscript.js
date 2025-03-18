window.addEventListener('DOMContentLoaded', function () {
    window.addEventListener('keydown', function (key) {
        if (!key.key) return;
        let keyvalue = key.key.toLowerCase();

        if (keyvalue == 'alt') {
            let message = { key: keyvalue, state: 'down' };
            if (chrome.runtime)
                chrome.runtime.sendMessage(null, message, (response) => {
                    console.log(response);
                });
            else console.log('no chrome runtime found');
        }
    });

    window.addEventListener('keyup', function (key) {
        if (!key.key) return;
        let keyvalue = key.key.toLowerCase();
        if (keyvalue == 'alt') {
            let message = { key: keyvalue, state: 'up' };
            if (chrome.runtime)
                chrome.runtime.sendMessage(null, message, (response) => {
                    console.log(response);
                });
            else console.log('no chrome runtime found');
        }
    });
});
