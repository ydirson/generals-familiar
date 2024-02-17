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
* specify armies to load using their ID (extracted from "share as
  link" in Army Forge).  Some examples are provided in the landing
  page.

### Plans and ideas for the future

* essential
  * let users select their own armies from a UI
  * provide help on keywords
  * support combined/joined units
  * support more than 2 armies
  * allow using community translations
* QoL
  * provide a menu with a few links (project's github, AF, rules)
  * let users directly paste the full URL provided by "share as link"
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
  * allow adding visual for each unit type and/or unit, to ease their
    fig identification (essential for opponent army)
  * add some stats to unit state (damage, mana, activated, exhausted...)
    and a way to change them
  * keep history of stat changes (to help check about forgotten stuff,
    and help battle reports)
  * proximity communication to share stats update, armies
  * quick switch of selection (for small lists) by e.g. swiping the
    drawer up/down
  * quick switch of selection for larger lists, by using e.g. an iconbar
    to jump to given unit type (use eg. unit acronyms for a first step)
  * assign an icon / get a pic to customize iconbar
  * some access to the rules for reference
  * allow some UI customization (font size...)
* later
  * WarFleets support (needs an API endpoint)
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
approach, and the [Leptonic component library](https://leptonic.dev/).
I'm not really happy yet with the generated WASM size, so maybe
technical choices will change at some point - at least this setup
makes it really easy to prototype things.

## Building locally / developping

The easiest for development is to launch in a terminal:
```
trunk serve --features dev
```

It will rebuild the project as you save files and trigger a browser
reload.  The `dev` feature will provide usable strack traces when the
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
generated `dist/index.html`, useful while developping (to trigger
immediate reload on build success and a "build error" overlay when
that kind of things happen).  You definitely don't want to deploy this
code, which will periodically retry to open a websocket to trunk for
this usage, so don't forget to use a final `trunk build` for this
purpose.
