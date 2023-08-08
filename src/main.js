
/*
#151515
#22221F
#93A3B1
 */

// console.log(lessons.js);

function current_url() {
}

export function get_lessons_json() {
    return lessons;
    // if status, if ok etc...
    // return await response.text()
    // return "";

    // .then(response => response.json())
    // .then(json => console.log(json));
    // var mydata = JSON.parse(window.lessons);

}

// https://yew.rs/docs/0.18.0/concepts/services/fetch
// https://crates.io/crates/gloo-net
export function fetch_with_timeout(url, options, timeout = 15_000) {
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


const lessons = {
"lessons": [
    {
        "name": "Tutorial",
        "path": "tutorial",
        "exercises": [
            {
                "info": "Here's an exercise with the answers covered. Hover over them to reveal.",
                "table": [
                    ["body", "|kāya|"],
                    ["time", "|kāla|"],
                ],
                "default_mode": "Censor",
            },
            {
                "info": "Click the arrow on the top right to go to the next lesson."
            }
        ]
    },
    {
        "name": "Introduction lesson",
        "path": "intro",
        "exercises": [
            {
                "title": "Long or Short",
                "info": "There are long and short vowels, however when a vowel is , the overall length can differ. Decide whether just the vowel tends towards long or short in speech.",
                "table": [
                    ["bha",   "|short|"],
                    ["ṭā",    "|long|"],
                    ["jjū",   "|short|"],
                    ["tthu",  "|short|"],
                    ["att",   "|long|"],
                    ["ti",    "|short|"],
                    ["ibh",   "|short|"],
                    ["nte",   "|long|"],
                    ["ett",   "|short|"],
                    ["ro",    "|long|"],
                    ["nti",   "|short|"],
                    ["saṁ",   "|long|"],
                ],
                "default_mode": "DropDown",
                "explanation": "e and o are long. axx->long, āxx->short. This excludes the aspirated marker, h.",
                "page": 4
            }
        ]
    },
    {
        "name": "Lesson 1 - First conjugation",
        "path": "1",
        "exercises": [
            {
                "title": "Conjugate Bhū",
                "info": "The conjugations for a 1st conjugation verb.",
                "table": [
                    ["person", "singular",  "plural"],
                    ["1st",    "bhav|āmi|", "bhav|āma|"],
                    ["2nd",    "bhav|asi|", "bhav|atha|"],
                    ["3rd",    "bhav|ati|", "bhav|anti|"],
                ],
                "default_mode": "Censor",
            }
        ]
    },
    {
        "name": "Lesson 2 - Nominative plural",
        "path": "2",
        "exercises": [
            {

            }
        ]
    },
    {
        "name": "Lesson 3 - Nom. and Acc.",
        "path": "3",
        "exercises": [

        ]
    },
    {
        "name": "Lesson 4 - Aorist",
        "path": "4",
        "exercises": [

        ]
    },
    {
        "name": "Lesson 5 - Pronouns",
        "path": "5",
        "exercises": [

        ]
    },
    {
        "name": "Lesson 6 - Imperative",
        "path": "6",
        "exercises": [

        ]
    },
    {
        "name": "Lesson 7 - Part participle",
        "path": "7",
        "exercises": [

        ]
    },

]}
