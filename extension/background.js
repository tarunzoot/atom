var webRequests = [];

chrome.webRequest.onBeforeSendHeaders.addListener((details) =>
// chrome.webRequest.onBeforeRequest.addListener((details) =>
{
    console.log(details)

    if (webRequests.length >= 50)
    {
        webRequests.pop();
    }

    webRequests.push({ url: details.url, requestHeaders: details.requestHeaders })
    chrome.storage.local.set({ httpRequests: webRequests })
}, {
    urls: [
        '<all_urls>'
    ],
    types: [
        'main_frame',
        'sub_frame',
        'xmlhttprequest'
    ]
}, [
    'requestHeaders',
    // 'requestBody',
    'extraHeaders'
]);


if ('onDeterminingFilename' in chrome.downloads)
    chrome.downloads.onDeterminingFilename.addListener(handleDownload);
else
    chrome.downloads.onCreated.addListener(handleDownload);


function toBinary(string)
{
    const codeUnits = new Uint16Array(string.length);
    for (let i = 0; i < codeUnits.length; i++)
    {
        codeUnits[i] = string.charCodeAt(i);
    }
    return btoa(String.fromCharCode(...new Uint8Array(codeUnits.buffer)));
}

function postData(jsonObject, id)
{
    let jsonString = JSON.stringify(jsonObject);
    console.log(jsonString);

    fetch(
        `http://127.0.0.1:2866/`,
        {
            method: 'POST',
            cache: 'no-cache',
            mode: 'no-cors',
            body: `${jsonString}\n<END>\n`
        }
    )
        .then((value) => value.text())
        .then((text) =>
        {
            chrome.downloads.cancel(id, (e) =>
            {
                console.log(`cancelled`);
            });

            console.log(text)
        }).catch(console.log);
}

function handleDownload(e)
{
    console.log(e);

    if (/^blob/.test(e.finalUrl || e.url)) return;

    let jsonObject = {
        url: e.finalUrl || e.url,
        size: e.fileSize > 0 ? e.fileSize : 0,
        file_name: e.filename.split('/').reverse().shift(),
        headers: {
            referer: e.referrer
        }
    };

    chrome.storage.local.get('httpRequests', (httpRequests) =>
    {
        let headers = {};

        let index = httpRequests.httpRequests.findIndex((i) => i.url == e.finalUrl);
        if (index != -1)
        {
            let requestHeaders = httpRequests.httpRequests[index].requestHeaders;
            for (let i = 0; i < requestHeaders.length; i++)
            {
                let values = Object.values(requestHeaders[i]);
                if (values.length >= 2)
                {
                    headers[values[0]] = values[1];
                }
            }
            jsonObject.headers = headers;
        }
        else
        {
            headers['referer'] = e.referrer;
        }

        postData(jsonObject, e.id);
    })
}