
/*
#151515
#22221F
#93A3B1
 */

function fetch_with_timeout(url, options, timeout = 10_000) {
    return Promise.race([
        fetch(url, options),
        new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout))
    ]);
}

export async function sleep(duration) {
    await new Promise(r => setTimeout(r, duration));
}

export function call() { // you may also do this in rust side

    var params = {

    };

    let arg = "";
    let result =
       fetch_with_timeout("https://..." + arg, {
            headers: {
                'Content-Type': "application/json",
            },
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        })
        .then(response => response.json())
        .then(response => {
            console.log("fetching");
        })
        .catch(error => { });
}

export function prefersDarkScheme() {
    return !!(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches)
}
