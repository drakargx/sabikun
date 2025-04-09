WIP

A collection of crates:
* schemas - Implements [Yomitan dictionary schemas](https://github.com/yomidevs/yomitan/tree/master/ext/data/schemas). Currently can verify individual .json files for the dictionary index, terms (including structured content), and kanji.
* yomi_dict_db - Interface for interacting with a SQLite database that contains tables for accessing yomitan dictionary tables. Not fully functional at the moment.
* gui - Not much here at this time. Uses Dioxus to render a WebView. Intention is to have this functioning similar to a texthook page, like [Renji's texthooker page](https://github.com/Renji-XD/texthooker-ui).
