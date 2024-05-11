var webRequests = [];

// chrome.webRequest.onHeadersReceived.addListener(
//     (details) => {
//         console.log('onHeadersReceived');
//         console.log(details);
//     },
//     {
//         urls: ['<all_urls>'],
//         types: ['main_frame', 'sub_frame'],
//     },
//     ['responseHeaders', 'extraHeaders']
// );

chrome.webRequest.onResponseStarted.addListener(
    (details) => {
        let index = webRequests.findIndex((i) => i.url === details.url);
        if (index != -1) {
            const contentLengthIndex = details.responseHeaders.findIndex(
                (f) => f.name.toLowerCase() == 'content-length'
            );
            const acceptRangesIndex = details.responseHeaders.findIndex(
                (f) => f.name.toLowerCase() === 'accept-ranges'
            );
            if (contentLengthIndex !== -1 && acceptRangesIndex !== -1) {
                if (
                    details.responseHeaders[contentLengthIndex].value > 0 &&
                    /bytes/.test(details.responseHeaders[acceptRangesIndex].value)
                ) {
                    webRequests[index].sequential = false;
                    chrome.storage.local.set({ httpRequests: webRequests });
                }
            }
        }
    },
    {
        urls: ['<all_urls>'],
        types: ['main_frame', 'sub_frame'],
    },
    ['responseHeaders', 'extraHeaders']
);

chrome.webRequest.onBeforeSendHeaders.addListener(
    (details) => {
        let index = webRequests.findIndex((i) => i.url === details.url);
        if (index != -1) {
            webRequests[index].requestHeaders = details.requestHeaders;
            chrome.storage.local.set({ httpRequests: webRequests });
        }
    },
    {
        urls: ['<all_urls>'],
        types: ['main_frame', 'sub_frame'],
    },
    ['requestHeaders', 'extraHeaders']
);

chrome.webRequest.onBeforeRequest.addListener(
    (details) => {
        if (webRequests.length >= 50) webRequests.pop();

        webRequests.push({
            url: details.url,
            requestHeaders: details.requestHeaders,
            body: details.requestBody,
            method: details.method,
            sequential: true,
        });
        chrome.storage.local.set({ httpRequests: webRequests });
    },
    {
        urls: ['<all_urls>'],
        types: [
            'main_frame',
            'sub_frame',
            // 'xmlhttprequest'
        ],
    },
    [
        // 'requestHeaders',
        'requestBody',
        'extraHeaders',
    ]
);

if ('onDeterminingFilename' in chrome.downloads) chrome.downloads.onDeterminingFilename.addListener(handleDownload);
else chrome.downloads.onCreated.addListener(handleDownload);

function toBinary(string) {
    const codeUnits = new Uint16Array(string.length);
    for (let i = 0; i < codeUnits.length; i++) {
        codeUnits[i] = string.charCodeAt(i);
    }
    return btoa(String.fromCharCode(...new Uint8Array(codeUnits.buffer)));
}

function postData(jsonObject, id) {
    let jsonString = JSON.stringify(jsonObject);
    console.log(jsonObject);

    fetch(`http://127.0.0.1:2866/`, {
        method: 'POST',
        cache: 'no-cache',
        mode: 'no-cors',
        body: `${jsonString}\n<END>\n`,
    })
        .then((value) => value.text())
        .then((text) => {
            chrome.downloads.cancel(id, (e) => {});
        })
        .catch(console.log);
}

function handleDownload(e) {
    if (/^blob/.test(e.finalUrl || e.url)) return;

    let jsonObject = {
        url: e.finalUrl || e.url,
        size: e.fileSize > 0 ? e.fileSize : 0,
        file_name: e.filename.split('/').reverse().shift(),
        method: 'GET',
        headers: {
            referer: e.referrer,
        },
        body: '',
        sequential: true,
    };

    chrome.storage.local.get('httpRequests', (httpRequests) => {
        let headers = {
            referer: e.referrer,
        };

        let index = httpRequests.httpRequests.findIndex((i) => i.url === e.finalUrl);
        if (index != -1) {
            let req = httpRequests.httpRequests[index];
            jsonObject.sequential = req.sequential;
            if (req.method === 'POST') {
                jsonObject.method = 'POST';
                jsonObject.body = new URLSearchParams(req.body.formData).toString();
            }
            let requestHeaders = httpRequests.httpRequests[index].requestHeaders || [];
            for (let i = 0; i < requestHeaders.length; i++) {
                let values = Object.values(requestHeaders[i]);
                if (values.length >= 2) {
                    headers[values[0]] = values[1];
                }
            }
            jsonObject.headers = headers;
        } else {
            headers['referer'] = e.referrer;
        }

        postData(jsonObject, e.id);
    });
}
