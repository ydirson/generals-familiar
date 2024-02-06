# General's Familiar

This project aims to provide a helper-in-your-pocket-phone app for
figurine wargame players, primarily targetting
[OnePageRules](https://onepagerules.com/) for now, and potentially
extended in the future to support other games.

## [Check the app in your browser](https://ydirson.github.io/generals-familiar/master/)

This is a standalone web app: once loaded in your browser the only
network interaction will be to load your army lists.  It is intended
to be used in landscape orientation.

You can also choose to download a prebuilt app as *Artifact* of a
*workflow run* from [the CI
page](https://github.com/ydirson/generals-familiar/actions/workflows/ci.yml),
or build it yourself.  You will still need a web server to allow your
browser to access it - if you choose to build it yourself, just run
`trunk serve` to build and run a server on the result.

## Functionnalities and Roadmap

### Currently available

* download two army lists from [the One Page Rules Army
  Forge](https://army-forge.onepagerules.com/)
* display face to face the details of the selected unit from each army

### Ideas for the future

* fix basic UI issues
  * [x] unit-details Drawers should be positionned relative to
        screen/viewport not page
  * [x] set app title from Rust
  * [x] window space outside app not covered by stylesheet, looks bad
        in Dark Mode
  * [x] too much vertical space used in Drawers
  * [ ] allow changing font size?
  * [x] allow closing Drawers
  * [ ] selected unit should be highlighted in list
  * [ ] better column-width usage in weapons list
  * [ ] add some transparency to drawers
* plumbing
  * [ ] flags for WASM size reduction
  * [ ] hunt for any unused features in deps
* essential
  * let users select their own armies
  * provide help on keywords
  * WarFleets support
  * support more than 2 armies
  * allow using community translations
* QoL
  * move game name to AppBar, consistency check accross armies
  * order units, grouping identical ones
  * ensure important list-level info is displayed, and not hidden by
    the drawer
  * extract meaningful army-level info for permanent display (aura
    available from units, (optionally) spells, ...)
  * see how we can determine which characteristic of displayed unit is
    particularly useful to highlight to avoid forgetting, taking the
    selected opponent unit into account (similarly, which ones we can
    dim because they're not useful, like "Hero" in GFF when no other
    friendly unit has a lower quality)
  * add some stats to unit state (damage, mana, activated, exhausted...)
    and a way to change them
  * keep history of stat changes (to help check about forgotten stuff,
    and help battle reports)
  * proximity communication to share stats update
  * quick switch of selection (for small lists) by e.g. swiping the
    drawer up/down
  * quick switch of selection for larger lists, by using e.g. an iconbar
    to jump to given unit type (use eg. unit acronyms for a first step)
  * assign an icon / get a pic to customize iconbar


## About this application

Technical details for the curious mind :)

The app itself is written in [Rust](https://rust-lang.org/) and
[compiled to Webassembly](https://rustwasm.github.io/), which allows
to build a web application using a language more fun than Javascript.

The UI is currently relying on the [Leptos
framework](https://leptos.dev/) to support a component-based reactive
approach, and the [Leptonic component library](https://leptonic.dev/).
I'm not really happy yet with the generated WASM size, so maybe
technical choices will change at some point - at least this setup
makes it really easy to prototype things.

## Building locally / developping

The easiest for development is to launch `trunk serve` in a terminal,
it will rebuild the project as you save files and trigger a browser
reload.

`trunk serve` and flexible deployment of the build result however
conflict on one point: flexible deployment pushes for using relative
hrefs for `index.html` to load the WASM and CSS files, and `trunk
serve` [does not like
that](https://github.com/trunk-rs/trunk/issues/697).

If you need this flexible deployment (as is done in the project's
GitHub CI), you will need to build the project using `trunk build
--public-url ./`.
