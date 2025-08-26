# General's Familiar

This project aims to provide a helper-in-your-pocket-phone app for
tabletop/figurine wargame players, with focus on games from
[OnePageRules](https://onepagerules.com/).

It was born from the primary desire to easily access information about
units from *both armies* without having to switch browser tabs (which
lacks fluidity in today's mobile browsers).

It is primarily meant to be displayed on a smartphone in landscape
orientation.

## Check the app in your browser

This is a standalone web app: once loaded in your browser the only
network interaction will be to load your army lists.

This app is not considered "fully ready" yet, the best balance for a
first contact should be the ["development"
version](https://ydirson.github.io/generals-familiar/dev), more are
accessible [here](https://ydirson.github.io/generals-familiar/).

You can also choose to download a prebuilt app as *Artifact* of a
*workflow run* from [the CI
page](https://github.com/ydirson/generals-familiar/actions/workflows/ci.yml),
or build it yourself.  You will still need a web server to allow your
browser to access it - if you choose to build it yourself, just run
`trunk serve` to build and run a server on the result.

## Functionalities and Roadmap

### Currently available

* download one or more army lists from [the One Page Rules Army
  Forge](https://army-forge.onepagerules.com/), allowing global view
  also on opponent army(ies) (optimized for 1-2 armies for now)
* show both unit lists in a dense format suitable to fit completely on
  screen for most armies; direct link to armies' Army Forge page for
  convenience
* regroup combined/joined units into a single display
* display face to face the details of the selected unit from each army
* specify armies to load using their ID (extracted from "share as
  link" in Army Forge).  Some examples are provided in the landing
  page.
* display unit-relevant common- and army-rules in unit details

### Plans and ideas for the future

* essential
  * let users select their own armies from a UI, let them paste the
    full URL provided by "share as link"
  * show spells when needed (requires fetching json army book)
  * (bug) on army removal, removal link for previous armies are not updated
* QoL
  * special rules
    * in DetailsDrawer, highlight occurrences of a rule when clicking
      its description
    * identify special-rules mentioned in special-rules descriptions
      (eg. Shadow-Protocol, Rending in Melee, Tunneler, Frenzy); may
      need manual tagging to handle mentions of opponent rules
      (eg. Rending)
    * easy way to highlight units with a given rule (eg. for Robot)
    * scroll to rule def on click on keyword
  * additional info
    * provide a menu with a few links (project's GitHub, AF, rules)
    * provide army links to army book (book name in the link requires
      fetching json army book)
    * extract meaningful army-level info for permanent display (aura
      available from units, (optionally) spells, ...)
    * see how we can determine which characteristic of displayed unit is
      particularly useful to highlight to avoid forgetting, taking the
      selected opponent unit into account (similarly, which ones we can
      dim because they're not useful, like "Hero" in GFF when no other
      friendly unit has a lower quality)
    * allow adding visual/icon for each unit type and/or unit, to ease their
      fig identification (essential for opponent army)
    * add some stats to unit state (damage, mana, activated, exhausted...)
      and a way to change them
    * keep history of stat changes (to help check about forgotten stuff,
      and help battle reports)
  * UI structuring
    * order units (replicating AF order currently requires fetching
      json army book)
    * regroup identical units
    * ensure important list-level info is displayed, and not hidden by
      the drawer
  * data management
    * keep (in local storage) a list of recently-used armies
    * keep (in local storage) a list of preferred armies for easy selection
    * allow keeping army/book data in local storage for offline usage
  * UX
    * quick switch of selection (for small lists) by e.g. swiping the
      drawer up/down
    * quick switch of selection for larger lists, by using e.g. an iconbar
      to jump to given unit type (use eg. unit acronyms for a first step)
    * allow some UI customization (font size...)
    * on army removal, avoid reloading armies after the removed one
    * [opr-rs] further merge display of combined units (best effect
      would likely be with merging into a single unit, doubling unit
      and attack counts)
* extra features
  * some access to the rules for reference
  * better support more than 2 armies
    * in 1v1 enough of the list display usually remain visible (if the
      lists have similar-enough width) between panels to change the
      selection (even though the unit names may not be visible enough
      despite transparency), but with 3 units the 2nd army is easily
      the only one accessible. Make sure the detailed armies are
      accessible with folders open.
    * in team play like 2v2 we would want to use left panel for more
      than the 1st army
    * in 1v1v1 we may want to show units for the 2 opponent armies
      instead of ours (use 2 different areas on unit list to open to
      the left or right?  use swipe with direction?)
    * at most a single panel should be displayed on each side at any
      given time (deselect other armies?)
  * allow using community translations
  * explore possible use of opr-af-to-tts output to improve presentation
  * proximity communication to share stats update, armies
  * WarFleets support (needs an API endpoint)
  * campaign mode support
* plumbing
  * flags for WASM size reduction
  * hunt for any unused features in deps
  * maybe use something lighter than `leptos_router` to get the query
    string, reactively

## About this application

Technical details for the curious mind :)

The app itself is written in [Rust](https://rust-lang.org/) and
[compiled to Webassembly](https://rustwasm.github.io/), which allows
to build a web application using a language more fun than Javascript.

The UI is currently relying on the [Leptos
framework](https://leptos.dev/) to support a component-based reactive
approach, and the [Thaw component library](https://crates.io/crates/thaw).
I'm not really happy yet with the generated WASM size, so maybe
technical choices will change at some point - at least this setup
makes it really easy to prototype things.

## Building locally / developing

The easiest for development is to launch in a terminal:
```
trunk serve --features dev
```

It will rebuild the project as you save files and trigger a browser
reload.  The `dev` feature will provide usable stack traces when the
app goes `panic!()`.

`trunk serve` and flexible deployment of the build result however
conflict on one point: flexible deployment pushes for using relative
hrefs for `index.html` to load the WASM and CSS files, and `trunk
serve` [does not like
that](https://github.com/trunk-rs/trunk/issues/697).

If you need this flexible deployment (as is done in the project's
GitHub CI), you will need to build the project using `trunk build
--public-url ./`.

Also note that `trunk serve` adds some additional javascript to the
generated `dist/index.html`, useful while developing (to trigger
immediate reload on build success and a "build error" overlay when
that kind of things happen).  You definitely don't want to deploy this
code, which will periodically retry to open a websocket to trunk for
this usage, so don't forget to use a final `trunk build` for this
purpose.
