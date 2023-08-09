
// done in rust with gloo-net
export function fetch_with_timeout(url, options, timeout = 15_000) {
    return Promise.race([
        fetch(url, options),
        new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout))
    ]);
}

export async function sleep(duration) {
    await new Promise(r => setTimeout(r, duration));
}

export function call() {

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

export function get_lessons_json() {
    return lessons;
}

// store everything in one js file because of how it gets built
/*
 * view exercise.rs Exercise struct to see the form of each exercise;
 * all parts are optional (to allow basic text in the middle of the page beyond exercises)
 * the possible default-mode values are the titles of ExerciseMode (Show, Censor (default), CensorByLetter, TypeField, DropDown)
 * and similarly for  "options_style": { "type": "..." } , the options are in table.rs OptionsStyle. ("Disabled", "All", "ByCol").
 *  if none is given, it predicts what it should be. Users can't change it after initialization
 * This is designed to technically work with tables that aren't grid shaped - no guarantees
 *
 */
const lessons = {
"lessons": [
    {
        "name": "Tutorial",
        "path": "tutorial",
        "exercises": [
            {
                "title": "click ",
            },

            {
                "info": "Here's an exercise with the answers covered. Hover over them to reveal.",
                "table_layout": {
                    "table": [
                        ["body", "|kāya|"],
                        ["time", "|kāla|"],
                    ],
                },
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
                "table_layout": {
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
                },
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
                "info": "The conjugations for a 1st conjugation verb \"to be\"",
                "table_layout": {
                    "table": [
                        ["person", "singular", "plural"],
                        ["1st", "bhav|āmi|", "bhav|āma|"],
                        ["2nd", "bhav|asi|", "bhav|atha|"],
                        ["3rd", "bhav|ati|", "bhav|anti|"],
                    ],
                }
            },
            {
                "title": "Verbs",
                "info": "These belong to the first conjugation",
                "table_layout": {
                    "table": [
                        ["root", "verb", "meaning"],
                        ["|kam|", "|upasaṁkamati|", "|he goes to, approaches|"],
                        ["|kam|", "|pakkamati|", "|he goes away|"],
                        ["|cu|", "|cavati|", "|he dies|"],
                        ["|jīv|", "|jīvati|", "|he lives|"],
                        ["|pass|", "|passati|", "|he sees|"],
                        ["|pucch|", "|pucchati|", "|he asks|"],
                        ["|bandh|", "|bandhati|", "|he binds|"],
                        ["|bhās|", "|bhāsati|", "|he says, speaks|"],
                        ["|bhū|", "|bhavati|", "|he is, there exists|"],
                        ["|vad|", "|vadati|", "|he says|"],
                        ["|sīd|", "|nisīdati|", "|he sits (down)|"],
                        ["|har|", "|harati|", "|he takes|"],
                        ["|har|", "|āharati|", "|he brings|"],
                        ["|hū|", "|hoti|", "|he is, there is|"]
                    ],
                    "default_mode": "DropDown",
                },
                "explanation": "",
                "page": 11,
            },
            {
                "title": "Nouns",
                "info": "Masculine nouns in -a in the nominative singular.",
                "table_layout": {
                    "table": [
                        ["noun",        "meaning"],
                        ["|upāsako|",   "|lay disciple|"],
                        ["|kāyo|",      "|body, substance|"],
                        ["|khattiyo|",  "|warrior, noble|"],
                        ["|gāmo|",      "|village|"],
                        ["|tathāgato|", "|thus-gone|"],
                        ["|devo|",      "|god, king|"],
                        ["|putto|",     "|son|"],
                        ["|puriso|",    "|man, person|"],
                        ["|brāhmaṅo|",  "|priest, brahman|"],
                        ["|maggo|",     "|road, way|"],
                        ["|manusso|",   "|human being, person|"],
                        ["|amanusso|",  "|non-human being|"],
                        ["|mahāmatto|", "|minister|"],
                        ["|loko|",      "|world, people, universe|"],
                        ["|samaṅo|",    "|ascetic, wanderer, philosopher|"],
                        ["|samayo|",    "|time, occasion|"]
                    ],
                    "default_mode": "DropDown",
                    // "options_style": { "ByCol": {} },
                },
                "explanation": "jqkbx",
                "page": 13,
            },
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
    // {
    //     "name": "Lesson 3 - Nom. and Acc.",
    //     "path": "3",
    //     "exercises": [
    //
    //     ]
    // },
    // {
    //     "name": "Lesson 4 - Aorist",
    //     "path": "4",
    //     "exercises": [
    //
    //     ]
    // },
    // {
    //     "name": "Lesson 5 - Pronouns",
    //     "path": "5",
    //     "exercises": [
    //
    //     ]
    // },
    // {
    //     "name": "Lesson 6 - Imperative",
    //     "path": "6",
    //     "exercises": [
    //
    //     ]
    // },
    // {
    //     "name": "Lesson 7 - Part participle",
    //     "path": "7",
    //     "exercises": [
    //
    //     ]
    // },

]}
