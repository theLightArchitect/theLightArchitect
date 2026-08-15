# Foundational Tool Palette

What real scholars, students, and Bible translators actually use — researched
Aug 2026 — and the tool palette Berean derives from it. Companion to
`ARCHITECTURE.md`; this is the data-and-capability roadmap for the Berean
Engine and the agents that sit on it.

## 1. How the three real audiences actually work

**Scholars / seminarians** (Logos, Accordance, BibleWorks-era workflow) build
a workspace around: a critical original-language text (NA28/UBS5/SBLGNT for
NT, BHS/BHQ for OT) → a tagged interlinear → one-click lemma lookup into a
serious lexicon (BDAG for Greek, HALOT for Hebrew) → concordance search over
every occurrence of the lemma → critical apparatus for variants → then
commentaries. The two daily-driver moves are **tap a word, get its
morphology + lexicon entry** and **show me every occurrence of this lemma**.

**Students / lay studiers** (Blue Letter Bible, BibleHub, STEP Bible, NET
Bible) do a lighter version of the same: parallel translations side by side,
Strong's-tagged interlinear, Treasury of Scripture Knowledge cross-references,
classic public-domain commentaries — plus a *method*. The dominant taught
method is **inductive study**: Observation ("what does the text say?") →
Interpretation ("what did it mean to its original audience?") → Application
("what does it mean for me?"), each viewed through historical, literary, and
theological lenses. That's the same epistemic layering Berean's root agent
already enforces — the method and our architecture agree.

**Translators** (Paratext, the SIL/UBS standard) add a workflow dimension the
consumer tools don't have: **biblical-terms consistency** (is this
Greek/Hebrew term rendered consistently across the whole translation?),
**back-translation** (an independent rendering to reveal how a reader will
actually understand the text), comprehension-question checking
(Transcelerator), and automated structural checks. The transferable insight:
translation quality is checked by *systematic cross-verification*, not by
trusting any single rendering — exactly Berean's verification-first posture.

## 2. The licensing reality (the decisive constraint)

The gold-standard scholarly resources are copyrighted and expensive to
license: BDAG, HALOT, Louw-Nida semantic domains, the NA28/UBS5 apparatus,
and major modern translations (ESV, NIV, NASB). A v1 built on those would be
gated on licensing deals before a single feature ships.

The good news: the open ecosystem has matured to the point where a genuinely
scholarly v1 is buildable with zero licensing negotiations:

| Layer | Open resource | License |
|---|---|---|
| English translation | **Berean Standard Bible (BSB)** | Public domain (CC0, since 2023) |
| More translations | WEB, KJV, ASV, etc. (open-bibles repo) | PD / libre |
| Tagged Greek NT | **STEPBible TAGNT**, unfoldingWord UGNT, SBLGNT | CC BY 4.0 / open / EULA-permissive |
| Tagged Hebrew OT | **STEPBible TAHOT**, unfoldingWord UHB (via OpenScriptures) | CC BY 4.0 / open |
| Greek lexicon | **TBESG (Abbott-Smith-based)**, Dodson | CC BY 4.0 / PD |
| Hebrew lexicon | **BDB** | PD |
| Syntax, semantic domains, glosses, referents | **MACULA Greek + Hebrew** (Clear.Bible) | open, per-layer |
| Manuscript transcriptions | **CNTR** — nearly every extant Greek NT ms. to AD 400 | open |
| Cross-references | **Treasury of Scripture Knowledge** (~500k), OpenBible.info compilation (~340k) | PD |
| Church fathers | **CCEL** Ante-Nicene / Nicene / Post-Nicene Fathers | PD |
| Creeds/confessions | Ancient creeds + historic Protestant confessions | PD (older translations) |
| Places/geography | **OpenBible.info Bible Geocoding** (every place, lat/lon, linked to Pleiades) | CC BY 4.0 |
| Proper names | STEPBible **TIPNR** (people/places disambiguated) | CC BY 4.0 |

Notable middle case: the **NET Bible's 60k+ translator notes** are free to
read and uniquely valuable (they explain *why* translators chose a
rendering) but not open-licensed — a candidate for a future permission
conversation, not a v1 dependency.

Strategy this implies: **build every tool contract against open data now;
treat licensed upgrades (BDAG/HALOT/NA28 apparatus) as drop-in dataset
swaps behind the same tool interface later.** The engine's tool contracts
never need to change — only the datasets behind them.

## 3. The palette

### Already in the engine — now with a designated backing dataset

| Tool | Backing data (v1) | Licensed upgrade path |
|---|---|---|
| `lookup_passage` | BSB + WEB/KJV/ASV | ESV/NIV/NASB via license |
| `lookup_lexicon` | TBESG + Dodson (Greek), BDB (Hebrew), Strong's | BDAG, HALOT |
| `lookup_crossrefs` | TSK + OpenBible 340k (curated); embeddings (ai_suggested) | — |
| `lookup_manuscript_variants` | CNTR transcriptions | NA28/UBS5 apparatus |
| `search_patristics` | CCEL ANF/NPNF | modern critical editions |
| `lookup_confession` | PD creed/confession texts | modern translations of same |
| `compare_translations` | the open translation set above | grows with licensed translations |

(`detect_pastoral_signal`, `read_journal`, `write_journal` are safety/
continuity tools — no scholarly dataset applies.)

### New tools the research says are missing

Ranked by how central they are to how people actually study:

1. **`get_interlinear`** — word-by-word aligned original + gloss + Strong's +
   morphology for a passage (TAGNT/TAHOT/MACULA). The single most-used
   original-language feature across every platform studied. Feeds the
   Passage Reader's tap-a-word interaction directly.
2. **`search_concordance`** — every occurrence of a lemma/Strong's number,
   with context snippets. The scholar's word-study workflow; pairs with
   `lookup_lexicon` the way BLB pairs Strong's with its concordance.
3. **`get_semantic_domain`** — words related by meaning-domain, not just
   etymology (MACULA semantic domains). This is what prevents the classic
   word-study fallacy of treating one Greek word as one English concept.
4. **`get_discourse_features`** — syntax trees, clause structure, discourse
   markers (MACULA). Scholar-altitude only; powers "why does this sentence
   turn on 'therefore'" answers with data instead of vibes.
5. **`lookup_place`** — geocoding + Pleiades-linked context for any biblical
   place (OpenBible Geocoding, TIPNR). Cheap to build, big payoff for
   historical-context answers and future map UI.
6. **`check_term_consistency`** — Paratext's biblical-terms idea turned
   outward: show every rendering of a given Hebrew/Greek term across a
   translation. This is the data behind honest "is this translation
   consistent here?" answers, and nothing consumer-facing offers it today.

### Method, not just data

The inductive method (observation → interpretation → application) should be
an explicit **guided-study mode** in the root agent — a workflow the agent
can walk a user through, Socratic-restraint style — rather than an engine
tool. The agent instruction already gestures at this; the research confirms
it's *the* taught method and worth naming as a first-class mode.

## 4. Suggested build order

1. **Corpus ingestion: BSB + TAGNT/TAHOT** → makes `lookup_passage`,
   `get_interlinear`, and `search_concordance` real. This is the unlock for
   everything else.
2. **TBESG/BDB lexicons** → `lookup_lexicon` real.
3. **TSK/OpenBible cross-refs** → `lookup_crossrefs` curated side real.
4. **MACULA layers** → semantic domains + discourse features.
5. **CNTR, CCEL, creeds, geocoding** → the depth tools.

## Sources

- [Logos vs Accordance comparison](https://learnofchrist.com/resources/compare/logos-vs-accordance) and [BDAG](https://learnofchrist.com/resources/bdag)/[HALOT](https://learnofchrist.com/resources/halot) reviews — scholar workflow
- [STEPBible-Data (CC BY 4.0)](https://github.com/STEPBible/STEPBible-Data) — TAGNT, TAHOT, TBESG, TIPNR
- [awesome-bible-data](https://github.com/jcuenod/awesome-bible-data) — curated open-data index
- [MACULA](https://www.tools.bible/tools/macula) — syntax/semantics/discourse layers
- [CNTR transcriptions](https://github.com/Center-for-New-Testament-Restoration/transcriptions) — manuscript data
- [BSB public domain release](https://bereanbible.com/) / [open-bibles](https://github.com/seven1m/open-bibles)
- [OpenBible cross-references](https://www.openbible.info/labs/cross-references/) and [Bible Geocoding Data](https://github.com/openbibleinfo/Bible-Geocoding-Data)
- [Abbott-Smith TEI project (PD)](https://github.com/translatable-exegetical-tools/Abbott-Smith), [Dodson lexicon (PD)](https://github.com/biblicalhumanities/Dodson-Greek-Lexicon)
- [CCEL Church Fathers](https://www.ccel.org/ccel/schaff/anf01.html) — PD patristics
- [Paratext](https://paratext.org/) ([SIL plan](https://paratext.org/paratext-training/tutorials/project-plans/sil-compact-plan/), [Transcelerator](https://software.sil.org/transcelerator/)) — translator workflow
- [Blue Letter Bible](https://learnofchrist.com/resources/blue-letter-bible), [BibleHub/BLB comparison](https://www.scriptureverse.app/blog/biblehub-vs-blue-letter-bible-vs-scriptureverse-comparison) — student tools
- Köstenberger & Fuhr, *Inductive Bible Study*; Bauer & Traina, *Inductive Bible Study* — method
