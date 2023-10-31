
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
                    ["Eng",    "Pāli"  ],
                    ["|body|", "|kāya|"],
                    ["|time|", "|kāla|"],
                ],
                "default_mode": "ClickReveal",
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
    "name": "Lesson 1 - 1st Conjugation",
    "path": "1",
    "exercises": [
        {
            "title": "Conjugate √bhū",
            "categories": ["Conjugation"],
            "info": "The conjugations for a 1st conjugation verb, \"to be\"",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular",  "plural"],
                    ["3rd",    "bhav|ati|", "bhav|anti|"],
                    ["2nd",    "bhav|asi|", "bhav|atha|"],
                    ["1st",    "bhav|āmi|", "bhav|āma|"],
                ],
            }
        },
        {
            "title": "Verbs",
            "categories": ["Verbs"],
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
            "categories": ["Vocab"],
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
    "name": "Lesson 2 - Nominative Plural",
    "path": "2",
    "exercises": [
        {
            "title": "Verbs",
            "categories": ["Verbs"],
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
            },
            "page": 17,
        },
        {
            "title": "Vocab",
            "categories": ["Vocab"],
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
                "key_col": 0,
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
            "categories": ["Declension"],
            "info": "Declension that also applies to brahman and rājan.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["meaning",     "fortunate one"],
                    ["stem",        "bhagav|ant|"  ],
                    ["nom. sing.",  "bhagav|ā|"    ],
                    ["acc. sing.",  "bhagav|ataṃ|" ],
                    ["nom. plur.",  "bhagav|ato|"  ],
                    ["acc. plur.",  "bhagav|ato|"  ],
                ],
            },
            "page": 20,
        },
        {
            "title": "Conjugate Dis",
            "categories": ["Conjugation"],
            "info": "The conjugations for a 7th conjugation verb, \"to teach\"",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"    ],
                    ["3rd",    "dese|ti|", "dese|nti|" ],
                    ["2nd",    "dese|si|", "dese|tha|" ],
                    ["1st",    "dese|mi|", "dese|ma|"  ],
                ],
            },
            "page": 21,
        },

        {
            "title": "Verbs",
            "categories": ["Verbs"],
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
            "categories": ["Vocab"],
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
                ],
                "key_col": 0,
            },
            "page": 22,
        },
    ]
},
{
    "name": "Lesson 4 - Aorist",
    "path": "4",
    "exercises": [
        {
            "title": "First Aorist Form Conjugation",
            "categories": ["Conjugation", "Aorist"],
            "info": "For upasaṃkamati - approaches.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "upasaṃkam|i|", "upasaṃkam|iṃsu|"],
                    ["2nd", "upasaṃkam|i|", "upasaṃkam|ittha|"],
                    ["1st", "upasaṃkam|iṃ|", "upasaṃkam|imha/imhā|"]
                ],
            },
            "page": 24,
        },
        {
            "title": "Second Aorist Form Conjugation",
            "categories": ["Conjugation", "Aorist"],
            "info": "For deseti - teaches.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "des|esi|", "des|uṃ|"],
                    ["2nd", "des|esi|", "des|ittha|"],
                    ["1st", "des|esiṃ|", "des|imha|"]
                ],
            },
            "page": 25,
        },
        {
            "title": "Third Aorist Form Conjugation",
            "categories": ["Conjugation", "Aorist"],
            "info": "For karoti - to make/do/work.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "ak|āsi|", "ak|aṃsu|"],
                    ["2nd", "ak|āsi|", "ak|attha|"],
                    ["1st", "ak|āsiṃ|", "ak|amhā|"]
                ],
            },
            "page": 25,
        },
        // {
        //     "title": "Aorist Form Verbs",
        //     "info": "Select the aorist form of the verb.",
        //     "table_layout": {
        //         "table": [
        //         ],
        //     },
        //     "page": 24,
        // },
        {
            "title": "√hū Aorist Conjugation",
            "categories": ["Conjugation", "Aorist"],
            "info": "For hoti - is.",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "ah|osi|", "ah|esuṃ|"],
                    ["2nd", "ah|osi|", "ah|uvattha|"],
                    ["1st", "ah|osiṃ|", "ah|umha|"]
                ],
            },
            "page": 26,
        },
        {
            "title": "Vocab",
            "categories": ["Vocab"],
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["|atha|",  "|then|"],
                    ["|ettha|", "|here|"],
                    ["|kho|",   "|indeed|"],
                    ["|ca|",    "|and|"],
                    ["|tadā|",  "|then|"],
                    ["|nāma|",  "|by name|"],
                    ["|bhūtapubbaṃ|", "|formerly|"],
                    ["|sace|",      "|if|"],
                    ["|kumāro|",    "|boy, prince|"],
                    ["|purohito|",  "|high priest|"],
                    ["|māṅavo|",    "|boy, young priest|"],
                    ["|rājaputto|", "|prince|"],
                    ["|sahāyo|",    "|friend|"],
                ],
                "key_col": 0,
            },
            "page": 26,
        },
    ]
},
        // {
        //     "title": "",
        //     "info": "",
        //     "exercise_level": "Important",
        //     "table_layout": {
        //         "table": [
        //         ],
        //     },
        //     "page": ,
        // },
{
    "name": "Lesson 5 - Pronouns",
    "path": "5",
    "exercises": [
        {
            "title": "First Person Personal Pronouns",
            "info": "stem ma(d)",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["case", "singular", "plural"],
                    ["nom.", "I |ahaṃ|", "we |mayaṃ|"],
                    ["acc.", "me |maṃ|", "us |amhe|"],
                ],
            },
        },
        {
            "title": "Second Person Personal Pronouns",
            "info": "stem ta(d)",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["case", "singular", "plural"],
                    ["nom.", "you |tvaṃ|", "you all |tumhe|"],
                    ["acc.", "you |taṃ|", "you all |tumhe|"],
                ],
            },
        },
        // {
        //     "title": "Third Person Personal Pronouns",
        //     "info": "stem ta(d)",
        //     "exercise_level": "Important",
        //     "table_layout": {
        //         "table": [
        //             ["case", "masc. sing.", "fem. sing.",   "masc. plur.",  "fem. plur."],
        //             ["nom.", "he |so|",     "she |sā|",     "they |te|",    "they |tā|"],
        //             ["acc.", "him |taṃ|",   "her |taṃ|",    "them |te|",    "them |tā|"],
        //         ],
        //     },
        //     "page": 28,
        // },
        {
            "title": "Third Person Personal Pronouns",
            "info": "stem ta(d)",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    // ["case", "m. sg.",    "f. sg.",    "n. sg.",          "m. pl.",    "f. pl.",     "n. pl."],
                    // ["nom.", "he |so|",   "she |sā|",  "it |taṃ or tad|", "they |te|", "they |tā|",  "they |tāni|"],
                    // ["acc.", "him |taṃ|", "her |taṃ|", "it |taṃ or tad|", "them |te|", "them |tā|",  "them |tāni|"],
                    ["gender number", "nom.",           "acc."],
                    ["masc. sing.", "he |so|",          "him |taṃ|"],
                    ["fem. sing.",  "she |sā|",         "her |taṃ|"],
                    ["neut. sing.", "it |taṃ/tad|",     "it |taṃ/tad|"],
                    ["masc. plur.", "they |te|",        "them |te|"],
                    ["fem. plur.",  "they |tā|",        "them |tā|"],
                    ["neut. plur.", "they |tāni|",      "them |tāni|"],
                ],
            },
            "page": 28,
        },
        {
            "title": "Demonstrative Pronoun - this, the",
            "categories": ["Declension"],
            "info": "idaṃ-",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["gender number", "nom.",  "acc."],
                    ["masc. sing.",   "|ayaṃ|",  "|imaṃ|"],
                    ["fem. sing.",    "|ayaṃ|",  "|imaṃ|"],
                    ["neut. sing.",   "|idaṃ|",  "|idaṃ|"],
                    ["masc. plur.",   "|ime|",   "|ime|"],
                    ["fem. plur.",    "|imā|",   "|imā|"],
                    ["neut. plur.",   "|imāni|", "|imāni|"],
                ],
            },
            "page": 30,
        },
        {
            "title": "√as Conjugation",
            "categories": ["Conjugation"],
            "info": "to be",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "|atthi|", "|santi|"],
                    ["2nd", "|asi|", "|attha|"],
                    ["1st", "|asmi/amhi|", "|amha/amhā|"]
                ],
                "default_mode": "HoverReveal",
            },
            "page": 31,
        },
        {
            "title": "\"be\" Disambiguation",
            "table_layout": {
                "table": [
                    ["he is", "precise meaning"],
                    ["ahosi", "|there is/exists|"],
                    ["bhavati", "|be/is|"],
                    ["atthi", "|is/becomes, eternal|"]
                ],
                "default_mode": "DropDown",
            },
        },
        {
            "title": "√vac Aorist Conjugation",
            "categories": ["Conjugation"],
            "info": "to say",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "|avoca|", "|avocuṃ|"],
                    ["2nd", "|avoca/avaca|", "|avocuttha|"],
                    ["1st", "|avocaṃ|", "|avocumha/ā|"]
                ],
                "default_mode": "HoverReveal",
            },
            "page": 31,
        },
        {
            "title": "Cases With Vocative",
            "categories": ["Declension"],
            "table_layout": {
                "table": [
                    ["case", "-a masc. sing.", "-a masc. plur."],
                    ["nom.", "-|o|",  "-|ā|"],
                    ["acc.", "-|aṃ|", "-|e|"],
                    ["voc.", "-|a|", "-|ā|"],
                ],
                "default_mode": "HoverReveal",
            },
            "page": 32,
        },
        {
            "title": "Vocab",
            "categories": ["Vocab"],
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["|apeti|",       "|goes away/from|"],
                    ["|tuṇhī|",       "|silent|"],
                    ["|pi|",          "|also, to|"],
                    ["|ha|",          "|indeed|"],
                    ["|issaro|",      "|lord, god|"],
                    ["|nirodho|",     "|cessation |"],
                    ["|paribbājako|", "|wanderer|"],
                    ["|mahārājā|",    "|great king|"],
                ],
                "key_col": 0,
            },
            "page": 32,
        },
    ]
},
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
{
    "name": "Lesson 8 - Further Use of Instrumental",
    "path": "8",
    "exercises": [
        {
            "title": "Instrumental Case Uses",
            "categories": ["Vocab"],
            "table_layout": {
                "table": [
                    ["type",             "example",                   "translation"],
                    ["|Accompaniment|",  "|brāmaṇena saddhiṃ|",       "|with the priest|"],
                    ["|Possession|",     "|sīlehi samannāgato|",      "|endowed with virtues|"],
                    ["|Filled|",         "|saddena|",                 "|(filled) with noise|"],
                    ["|Cause|",          "|bhagavatā vādena kupito|", "|angry at Buddha’s statement|"],
                    ["|Cause|",          "|cīvarena santuṭṭho|",      "|satisfied with the robe|"],
                    ["|Cause|",          "|atthena|",                 "|because of that matter|"],
                    ["|Cause|",          "|iminā p' aṅgena|",         "|(don't go) for this reason|"],
                    ["|Equality|",       "|samasamo vaṇṇena|",        "|quite equal in beauty|"],
                    ["|Equality|",       "|purisena purisaṃ|",        "|a man with a man|"],
                    ["|Price|",          "|sahassena|",               "|for a thousand|"],
                    ["|Way|",            "|dvārena|",                 "|by the gate|"],
                    ["|Direction|",      "|uttarāya|",                "|from north|"],
                    ["|Manner|",         "|iminā|",                   "|in this way|"],
                    ["|Manner|",         "|kāyena|",                  "|through body|"],
                    ["|Manner|",         "|santena|",                 "|calmly|"],
                    ["|Manner|",         "|kicchena|",                "|with difficulty|"],
                    ["|Means|",          "|dānena|",                  "|by giving|"],
                    ["|Vehicle|",        "|yānena|",                  "|by carriage|"],
                    ["|Motive|",         "|gāravena|",                "|through respect|"],
                    ["|Time|",           "|aparena samayena|",        "|after some time|"],
                    ["|Time|",           "|tena samayena|",           "|at that time|"],
                    ["|Time|",           "|accayena|",                "|through the passage|"],
                    // ["|Age|",            "||",                        "||"],   source's typo
                    ["|Measure|",        "|āyāmena|",                 "|in length|"],
                    ["|Measure|",        "|jannumattena|",            "|knee-deep|"],
                    ["|Classification|", "|Gotamo gottena|",          "|a Gotama by clan|"],
                    ["|Classification|", "|jātivādena|",              "|in respect of class|"],
                    ["|Dissociation|",   "|adaṇḍena|",                "|without force|"],
                    ["|Dissociation|",   "|aññatra brāhmaṇena|",      "|except for the priest|"],
                ],
                "key_col": 1,
            },
            "page": 44,
        },
        {
            "title": "gam Present Participles",
            "categories": ["Declension"],
            "info": "going",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["case",   "singular",   "plural"],
                    ["nom.",   "gacch|aṃ/anto|", "gacch|antā|"],
                    ["acc.",   "gacch|antaṃ|",   "gacch|ante|"],
                    ["instr.", "gacch|atā|",     "gacch|antehi|"],
                ],
            },
            "page": 46,
        },
        {
            "title": "bhavant Pronoun",
            "categories": ["Declension"],
            "info": "you, sir, your honor",
            "table_layout": {
                "table": [
                    ["case",   "masc. sing.", "masc. plur."],
                    ["nom.",   "|bhavaṃ|",    "|bhavanto/bhonto|"],
                    ["acc.",   "|bhavantaṃ|", "|bhavante|"],
                    ["instr.", "|bhotā|",     "|bhavantehi|"],
                    ["voc.",   "|bho|",       "|bhonto|"],
                ],
                "default_mode": "HoverReveal",
            },
            "page": 47,
        },

    ]
},
{
    "name": "Lesson - 9 Passive Conjugation",
    "path": "9",
    "exercises": [
        {
            "title": "√(p)pa-hā Present Indicative Passive Conjugation",
            "categories": ["Conjugation"],
            "info": "pahita, give up --> it is given up",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "pah|īyati|", "pah|īyanti|"],
                    ["2nd", "pah|īyasi|", "pah|īyata|"],
                    ["1st", "pah|īyāmi|", "pah|īyama|"]
                ],
                "key_col": 0,
            },
            "explanation": "passive transformation: add ya or tya to the root, which often loses its last vowel.",
            "page": 51,
        },
        {
            "title": "-ā Feminine Noun Ending",
            "categories": ["Declension"],
            "info": "for noun, kathā",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["case",   "singular",   "plural"],
                    ["nom.",   "kath|ā|", "kath|ā/āyo|"],
                    ["acc.",   "kath|aṃ|",   "kath|āyo|"],
                    ["instr.", "kath|āya|",     "kath|āhi|"],
                ],
            },
            "page": 46,
        },
    ]
},
// {
//     "name": "Lesson - 10 Future Tense",
//     "path": "10",
//     "exercises": [
//
//     ]
// },
{
    "name": "Lesson - 11 Adjectives",
    "path": "11",
    "exercises": [
        {
            "title": "√man Third Conjugation",
            "categories": ["Conjugation"],
            "info": "pahita, give up --> it is given up",
            "exercise_level": "Important",
            "table_layout": {
                "table": [
                    ["person", "singular", "plural"],
                    ["3rd", "maññ|ati|", "maññ|anti|"],
                    ["2nd", "maññ|asi|", "maññ|atha|"],
                    ["1st", "maññ|āmi|", "maññ|āma|"]
                ],
                "key_col": 0,
            },
            "explanation": "forms like passive with first conjugation endings: (man + ya = mañña).",
            "page": 51,
        },    ]
},
// {
//     "name": "Lesson - 12 Dative",
//     "path": "12",
//     "exercises": [
//
//     ]
// },
// {
//     "name": "Lesson - 13",
//     "path": "13",
//     "exercises": [
//
//     ]
// },
// {
//     "name": "Lesson - 14",
//     "path": "14",
//     "exercises": [
//
//     ]
// },



        //     "title": "",
        //     "categories": [""],
        //     "info": "",
        //     "exercise_level": "Important",
        //     "table_layout": {
        //         "table": [
        //             ["||", "||"],
        //         ],
        //         "key_col": 0,
        //     },
        //     "page": 0,
        // },

//     {
//         "title": "anonymous",
//         "exercises": [
//
//         ]
//     }


]}
