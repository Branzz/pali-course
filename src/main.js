
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

/**
 * returns the width of child text of any DOM node as a float
 */
function getTextWidth(e) {
    const canvas = getTextWidth.canvas || (getTextWidth.canvas = document.createElement("canvas"));
    const context = canvas.getContext("2d");
    const font = window.getComputedStyle(e, null).getPropertyValue('font');
    const text = e.value;
    context.font = font;
    const textMeasurement = context.measureText(text);
    return textMeasurement.width;
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
 * names and titles MUST be unique
 *
 * Replicate the textbook as close as possible
 * Titles should be short and in Title Case
 * The |markers| don't have to be around the entire cell ("pi |su|kkha")
 * spell Pāli with a capital and ā. Sentences start with capital unless it's Pāli. use the diacritics, only use ṃ (to match textbook)
 * The explanation should be something that would spoil the problem when revealed such as a grammar rule
 *   or some sort of exception
 * End every 'fuller' sentence with a period
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
const lessons = // { "courses": [...]}
{ "lessons": [
{
    "name": "Tutorial",
    "path": "tutorial",
    "exercises": [
        {
            "title": "Layout",
            "info": "Here's an exercise with the answers covered. Click on them to reveal one-by-one." +
                " Then, see the drop down for the other modes." +
                " They each have their own advantage such as omitting the other options or other already given answers." +
                " Some let you check your answers." +
                " You can also press TAB to switch between cells.",
            "table_layout": {
                "table": [
                    ["Eng",  "Pāli"  ],
                    ["|body|", "|kāya|"],
                    ["|time|", "|kāla|"],
                ],
                "key_col": 0,
            },
            "explanation": "\"body\" in Pāli is kāya. You can guess what \"time\" is :)",
        },
        {
            "title": "↑",
            "info": "Click the box above to show the explanation for the exercise.",
            // "table_layout": {
            //     "table": [
            //         ["Exercise mode"]
            //         ["Reveal all"]
            //         ["Hover reveal"]
            //         ["Click reveal"]
            //         ["Reveal by letter"]
            //         ["Enter text"]
            //         ["Drop down"]
            //     ],
            // },
        },
        {
            "title": "Input Method",
            "exercise_level": "Important",
            "info": "Here are the alternate ways to type the accented Pāli characters on this site. The star means that this is an \"important\" exercise (some exercises are less useful details, while some are more valuable to learning Pāli)",
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
            "title": "Finally,",
            "info": "Click the sun/moon on the top right to switch to dark/light mode. Hovering near the title lets you link to the exercise. Click the arrow on the top right to go to the first lesson."
        }
    ]
},
{
    "name": "Introduction lesson",
    "path": "intro",
    "exercises": [
        {
            "title": "Long or Short",
            "info": "Decide whether just the vowel tends towards long or short in speech and whether the vowel's syllable tends towards \"long\" or \"short\".",
            "table_layout": {
                "table": [
                    ["part",  "vowel length", "syllable length"],
                    ["bha",   "|short|",      "|short|"],
                    ["ṭā",    "|long|",       "|long|"],
                    ["jjū",   "|long|",       "|long|"],
                    ["tthu",  "|short|",      "|short|"],
                    ["att",   "|short|",      "|long|"],
                    ["aṅgh",  "|short|",      "|long|"],
                    ["ti",    "|short|",      "|short|"],
                    ["saṃ",   "|short|",      "|long|"],
                    ["ibh",   "|short|",      "|short|"],
                    ["umh",   "|short|",      "|long|"],
                    ["mett",  "|short|",      "|long|"],
                    ["ro",    "|long|",       "|long|"],
                    /* v - vowel, s - short vowel, l - long vowel, c - consonant (including aspirated)
                     * vcc -> short, long
                     * vṃ -> short, long
                     * sc -> short, short
                     * lc -> long, long
                     */
                ],
                "default_mode": "DropDown",
            },
            "explanation": "e and o are long. Vowels are short if before double asp./unasp. consonant or ṃ. mh isn't a consonant.",
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
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular",  "plural"],
                    ["1st",    "bhav|āmi|", "bhav|āma|"],
                    ["2nd",    "bhav|asi|", "bhav|atha|"],
                    ["3rd",    "bhav|ati|", "bhav|anti|"],
                ],
                "default_mode": "HoverReveal",
            }
        },
        {
            "title": "Verbs",
            "info": "These are first conjugation verbs in the 3rd person singular.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["root",    "verb",           "meaning (one...)"],
                    ["|kam|",   "|upasaṃkamati|", "|goes to, approaches|"],
                    ["|kam|",   "|pakkamati|",    "|goes away|"],
                    ["|cu|",    "|cavati|",       "|dies|"],
                    ["|jīv|",   "|jīvati|",       "|lives|"],
                    ["|pass|",  "|passati|",      "|sees|"],
                    ["|pucch|", "|pucchati|",     "|asks|"],
                    ["|bandh|", "|bandhati|",     "|binds|"],
                    ["|bhās|",  "|bhāsati|",      "|says, speaks|"],
                    ["|bhū|",   "|bhavati|",      "|is, exists|"],
                    ["|vad|",   "|vadati|",       "|says|"],
                    ["|sīd|",   "|nisīdati|",     "|sits (down)|"],
                    ["|har|",   "|harati|",       "|takes|"],
                    ["|har|",   "|āharati|",      "|brings|"],
                    ["|hū|",    "|hoti|",         "|is|"]
                ],
                "key_col": 1,
            },
            "page": 11,
        },
        {
            "title": "Vocab",
            "info": "Masculine nouns in -a in the nominative singular.",
            "exercise_level": "Important",
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
                    ["|manusso|",   "|human, person|"],
                    ["|amanusso|",  "|non-human being|"],
                    ["|mahāmatto|", "|minister|"],
                    ["|loko|",      "|world, people|"],
                    ["|samaṇo|",    "|ascetic, wanderer|"],
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
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["root",     "verb",        "meaning (one...)"],
                    ["|vis|",    "|pavisati|",  "|enters|"],
                    ["|phus|",   "|phusati|",   "|touches, reaches, attains|"],
                    ["|is|",     "|icchati|",   "|wishes, desires|"],
                    ["|gam|",    "|gacchati|",  "|goes|"],
                    ["|gam|",    "|āgacchati|", "|comes|"],
                    ["|(ṭ)ṭhā|", "|tiṭṭhati|",  "|stands, remains, stays|"],
                    ["|dā|",     "|deti|",      "|gives|"],
                    ["|hā|",     "|pajahati|",  "|gives up, renounces|"],
                    ["|(v)vaj|", "|pabbajati|", "|goes forth|"],
                    ["|(j)jhe|", "|jhyāti|",    "|meditates|"],
                    ["|i|",      "|eti|",       "|goes|"],
                    ["|i|",      "|upeti|",     "|goes|"]
                ],
                "key_col": 1,
            },
            "explanation": "Consonants will become \"assimilated\" with each other like s + ch -> cch",
            "page": 16,
        },
        {
            "title": "Cases",
            "info": "This is the form a noun takes when it's a direct object, generally when it's undergoing some action or as an attribute for another accusative object. See the table of stem translations below.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["stem / case", "-a masc. sing.", "-a masc. plur."],
                    ["nom.", "-|o|",  "-|ā|"],
                    ["acc.", "-|aṃ|", "-|e|"],
                ],
                "default_mode": "HoverReveal",
            },
            "page": 17,
        },
        {
            "title": "Vocab",
            // "info": "Masculine nouns in -a in the nominative singular.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["noun",     "meaning"],
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
{
    "name": "Lesson 3 - 7th Conjugation",
    "path": "3",
    "exercises": [
        {
            "title": "Declension",
            "info": "Declension that also applies to brahman and rājan.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["meaning",     "fortunate one",],
                    ["stem",        "bhagav|ant|",  ],
                    ["nom. sing.",  "bhagav|ā|",    ],
                    ["acc. sing.",  "bhagav|ataṃ|", ],
                    ["nom. plur.",  "bhagav|ato|",  ],
                    ["acc. plur.",  "bhagav|ato|",  ]
                ],
                "default_mode": "HoverReveal",
            },
            "page": 20,
        },
        {
            "title": "Conjugate Dis",
            "info": "The conjugations for a 7th conjugation verb, \"to teach\"",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"    ],
                    ["1st",    "dese|mi|", "dese|ma|"  ],
                    ["2nd",    "dese|si|", "dese|tha|" ],
                    ["3rd",    "dese|ti|", "dese|nti|" ]
                ],
                "default_mode": "HoverReveal",
            },
            "page": 21,
        },

        {
            "title": "Verbs",
            "info": "7th conjugation verbs.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["root",    "verb",          "meaning (one...)"],
                    ["|kath|",	 "|katheti|",       "|relates, tells|"],
                    ["|kam|",    "|kāmeti|",        "|loves|"],
                    ["|chaḍḍ|",	 "|chaḍḍeti|",      "|throws away, abandons|"],
                    ["|(ñ)ñap|", "|aññāpeti|",      "|prepares, declares|"],
                    ["|dhar|",	 "|dhareti|",       "|holds, wears, has, accepts, remembers|"],
                    ["|mant|",	 "|manteti|",       "|takes counsel, discusses|"],
                    ["|mant|",	 "|āmanteti|",      "|addresses|"],
                    ["|vañc|",	 "|vañceti|",       "|deceives|"],
                    ["|vad|",	 "|abhivādeti|",    "|salutes, greets, takes leave|"],
                    ["|vās|",    "|nivāseti|",      "|dresses|"],
                    ["|vid|",    "|paṭisaṃvedeti|", "|feels, experiences|"],
                    ["|veṭh|",	 "|ibbeṭheti|",	    "|untwists, unravels, explains, rebuts|"]
                ],
                "key_col": 1,
            },
            "page": 21,
        },
        {
            "title": "Vocab",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["|kālo|",  	"|time|"],
                    ["|paccayo|",   "|condition, cause|"],
                    ["|bhāro|", 	"|burden, load|"],
                    ["|lābho|", 	"|gain|"],
                    ["|vipāko|",    "|result|"],
                    ["|vihāro|",    "|life, dwelling|"],
                    ["|hattho|",    "|hand|"]
                ]
            },
            "page": 22,
        },
    ]
},
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

//     {
//         "title": "anonymous",
//         "exercises": [
//
//         ]
//     }

]}
