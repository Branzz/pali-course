
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

// store everything in one js file because of how the entire project gets built (it doesn't like to work with another file's path)

/**
 * Basic Guide
 *
 * Replicate the textbook as close as possible
 * Titles should be short and in Title Case
 * The |markers| don't have to be around the entire cell ("pi |su|kkha")
 * spell Pāli with a capital and ā
 * The explanation should be something that would spoil the problem when revealed such as a grammar rule
 *   or some sort of exception
 * End any sentence with a period
 * You can't insert HTML into the strings (not my choice)
 * every field is optional (this allows basic text in the middle of the page without even a table)
 *  (except table_layout must have a table)
 *
 * View exercise.rs Exercise struct to see the form of "default_mode"
 *  The possible default-mode values are the names of ExerciseMode's (Show, HoverReveal, ClickReveal, CensorByLetter, TypeField, DropDown)
 * and similarly for "options_style", the options are in table.rs OptionsStyle.
 *  ...=(Disabled, All, ByCol).
 *  The options style details are predicted. Users can't change the options style.
 * This is technically designed to include tables that aren't grid shaped - no guarantees (it creates rows from the right)
 *
 * Don't worry about indentation and whatever, I can easily reformat it. The most helpful thing if anything would be
 * just copying and reformatting the text over from the textbook.
 *
 */
const lessons = { "lessons": [
{
    "name": "Tutorial",
    "path": "tutorial",
    "exercises": [
        {
            "title": "Layout",
        },

        {
            "info": "Here's an exercise with the answers covered. Hover over them to reveal. Then, see all the other modes in the drop down; some let you check your answers.",
            "table_layout": {
                "table": [
                    ["body", "|kāya|"],
                    ["time", "|kāla|"],
                ],
            },
            "explanation": "\"body\" in Pāli is \"kāya\", and I think you can guess what \"time\" is.",
        },
        {
            "info": "Click the box above to show the explanation for the exercise.",
        },
        {
            "title": "Input Method",
            // The star here means it's an important lesson (some exercises are less useful details, while some are more valuable to learning Pāli)
            "info": "Here are the alternate ways to type the accented Pāli characters on this site.",
            "table_layout": {
                // match the const in cell.rs
                "table": [
                    ["aa", "ā"],
                    ["ii", "ī"],
                    ["uu", "ū"],
                    [".t", "ṭ"],
                    [".d", "ḍ"],
                    ["`n", "ṅ"],
                    ["~n", "ñ"],
                    [".n", "ṇ"],
                    [".m", "ṃ"],
                    [".l", "ḷ"],
                ]
            }
        },
        {
            "info": "Click the sun/moon on the top right to switch to dark/light mode. Click the arrow on the top right to go to the first lesson."
        }
    ]
},
{
    "name": "Introduction lesson",
    "path": "intro",
    "exercises": [
        {
            "title": "Long or Short",
            "info": "Decide whether just the vowel tends towards long or short in speech and whether the entire construction tends towards \"heavy\" or \"light\".",
            "table_layout": {
                "table": [
                    ["part",  "vowel length"],
                    ["bha",   "|short|"     ],
                    ["ṭā",    "|long|"      ],
                    ["jjū",   "|short|"     ],
                    ["tthu",  "|short|"     ],
                    ["att",   "|short|"     ],
                    ["ti",    "|short|"     ],
                    ["ibh",   "|short|"     ],
                    ["nte",   "|long|"      ],
                    ["mett",  "|short|"     ],
                    ["ro",    "|long|"      ],
                    ["nti",   "|short|"     ],
                    ["saṃ",   "|short|"     ],
                ],
                "default_mode": "DropDown",
            },
            "explanation": "e and o are long unless before double consonant.",
            "page": 4,
        }
    ]
},
{
    "name": "Lesson 1 - First conjugation",
    "path": "1",
    "exercises": [
        {
            "title": "Conjugate Bhū",
            "info": "The conjugations for a 1st conjugation verb, \"to be\"",
            "table_layout": {
                "table": [
                    ["person", "singular",  "plural"],
                    ["1st",    "bhav|āmi|", "bhav|āma|"],
                    ["2nd",    "bhav|asi|", "bhav|atha|"],
                    ["3rd",    "bhav|ati|", "bhav|anti|"],
                ],
                "default_mode": "HoverReveal",
                "options_style": "Disabled",
            }
        },
        {
            "title": "Verbs",
            "info": "These belong to the first conjugation.",
            "table_layout": {
                "table": [
                    ["root",    "verb",           "meaning"],
                    ["|kam|",   "|upasaṃkamati|", "|he goes to, approaches|"],
                    ["|kam|",   "|pakkamati|",    "|he goes away|"],
                    ["|cu|",    "|cavati|",       "|he dies|"],
                    ["|jīv|",   "|jīvati|",       "|he lives|"],
                    ["|pass|",  "|passati|",      "|he sees|"],
                    ["|pucch|", "|pucchati|",     "|he asks|"],
                    ["|bandh|", "|bandhati|",     "|he binds|"],
                    ["|bhās|",  "|bhāsati|",      "|he says, speaks|"],
                    ["|bhū|",   "|bhavati|",      "|he is, there exists|"],
                    ["|vad|",   "|vadati|",       "|he says|"],
                    ["|sīd|",   "|nisīdati|",     "|he sits (down)|"],
                    ["|har|",   "|harati|",       "|he takes|"],
                    ["|har|",   "|āharati|",      "|he brings|"],
                    ["|hū|",    "|hoti|",         "|he is, there is|"]
                ],
                "key_col": 1,
            },
            "page": 11,
        },
        {
            "title": "Vocab",
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
                    ["|brāhmaṇo|",  "|priest, brahman|"],
                    ["|maggo|",     "|road, way|"],
                    ["|manusso|",   "|human being, person|"],
                    ["|amanusso|",  "|non-human being|"],
                    ["|mahāmatto|", "|minister|"],
                    ["|loko|",      "|world, people, universe|"],
                    ["|samaṇo|",    "|ascetic, wanderer, philosopher|"],
                    ["|samayo|",    "|time, occasion|"]
                ],
                "key_col": 0,
                "shuffle_rows": true,
            },
            "page": 13,
        },
    ]
},
{
    "name": "Lesson 2 - Nominative plural",
    "path": "2",
    "exercises": [
        {
            "title": "Verbs",
            "info": "These have irregular stems.",
            "table_layout": {
                "table": [
                    ["root",     "verb",        "meaning"],
                    ["|vis|",    "|pavisati|",  "|he enters|"],
                    ["|phus|",   "|phusati|",   "|he touches, reaches, attains|"],
                    ["|is|",     "|icchati|",   "|he wishes, desires|"],
                    ["|gam|",    "|gacchati|",  "|he goes|"],
                    ["|gam|",    "|āgacchati|", "|he comes|"],
                    ["|(ṭ)ṭhā|", "|tiṭṭhati|",  "|he stands, remains, stays|"],
                    ["|dā|",     "|deti|",      "|he gives|"],
                    ["|hā|",     "|pajahati|",  "|he gives up, he renounces|"],
                    ["|(v)vaj|", "|pabbajati|", "|he goes forth|"],
                    ["|(j)jhe|", "|jhyāti|",    "|he meditates|"],
                    ["|i|",      "|eti|",       "|he goes|"],
                    ["|i|",      "|upeti|",     "|he goes to|"]
                ],
            },
            "explanation": "Consonants will become \"assimilated\" with each other like s + ch -> cch",
            "page": 16,
        },
        {
            "title": "Cases",
            "info": "This is the form a noun takes when it's a direct object, generally when it's undergoing some action or as an attribute for another accusative object. See the table of stem translations below.",
            "table_layout": {
                "table": [
                    ["stem / case", "-a masc. sing.", "-a masc. plur."],
                    ["nom.", "|-o|",  "|-ā|"],
                    ["acc.", "|-aṃ|", "|-e|"],
                ],
            },
            "page": 17,
        },
        {
            "title": "Vocab",
            // "info": "Masculine nouns in -a in the nominative singular.",
            "table_layout": {
                "table": [
                    ["|aggo|",   "|top|"],
                    ["|attho|",  "|prosperity, meaning...|"],
                    ["|dhammo|", "|true, natural; doctrine...|"],
                    ["|patto|",  "|bowl|"],
                    ["|pamādo|", "|negligence, pastime|"],
                    ["|piṇḍo|",  "|alms|"],
                    ["|bhavo|",  "|existence, good fortune|"],
                    ["|vādo|",   "|debate, argument, statement|"],
                    ["|satto|",  "|being, creature|"],
                    ["|saddo|",  "|noise, report|"],
                    ["|sugato|", "|well-gone|"]
                ],
            }
        },

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
