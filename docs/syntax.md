# Plotum syntax (v1)

Plotum diagrams are plain text files with the `.plotum` extension. The text is the
single source of truth: the visual editor rewrites it, so keep it readable and
diff-friendly.

Design rules:

- **One line, one idea**: a node, a link or an option.
- **Readable without the tool**: `api -> db : SELECT` should make sense on its own.
- **Smart defaults**: a minimal diagram fits in 3 lines, style comes from the theme.

## Minimal diagram

```plotum
# plotum 1
client -> api : GET /users
api -> db : SELECT
```

Nodes used in a link are created automatically, so this is a complete diagram.

## File structure

A file is read top to bottom, in this order:

1. Version line: `# plotum 1`
2. Header: `title`, `subtitle`, `footer`
3. Theme: the `theme:` block
4. Sections, nodes, links and notes

```plotum
# plotum 1
title: Checkout
subtitle: How a payment is processed
footer: plotum.dev

theme:
  colors:
    brand: #6D28D9

section info "Payment"
  shop [Shop | Next.js]
  psp [Payment provider]
  shop -> psp {brand} : charge
```

## Version line

The first line declares the syntax version. `# plotum 1` covers every
1.x version of the engine.

```plotum
# plotum 1
```

## Comments

```plotum
// a single-line comment

/* a comment
   on several lines */
```

## Header

| Element  | Syntax           | Example                 |
| -------- | ---------------- | ----------------------- |
| Title    | `title: text`    | `title: Payment`        |
| Subtitle | `subtitle: text` | `subtitle: Idempotency` |
| Footer   | `footer: text`   | `footer: @jonh-doe`         |

```plotum
title: Payment
subtitle: Idempotency keys
footer: @jonh-doe
```

## Nodes

| Element | Syntax                  | Example                |
| ------- | ----------------------- | ---------------------- |
| Circle  | `id (Label)`            | `user (Customer)`      |
| Box     | `id [Label]`            | `api [API]`            |
| Box     | `id [Label \| subtext]` | `api [API \| Node.js]` |

```plotum
user (Customer)
api [API | Node.js]
db [PostgreSQL | primary]
```

- Identifiers are case-sensitive and may contain accents and dashes: `user-db`, `élève`.
- Use `\n` for a line break inside a label: `api [Public API\nv2]`.
- A node used in a link without being declared is created automatically. It can
  also be declared later in the file.

## Links

| Stroke     | Forward   | Backward  | Both ways  |
| ---------- | --------- | --------- | ---------- |
| Solid      | `a -> b`  | `a <- b`  | `a <-> b`  |
| Dashed     | `a --> b` | `a <-- b` | `a <--> b` |
| Dotted     | `a ..> b` | `a <.. b` | `a <..> b` |
| Plain line | `a --- b` |           |            |

A plain line has no arrowhead, so it has no direction.

| Element | Syntax          | Example             |
| ------- | --------------- | ------------------- |
| Label   | `a -> b : text` | `a -> b : HTTP 200` |

```plotum
client -> api : GET /orders
api --> cache : lookup
api ..> logs : async
api --- sidecar
api <-> db : pool
```

Links between nodes of different sections are allowed.

## Sections

A section groups nodes under a heading. Its type sets its look.

```plotum
section <type> "Label"
```

| Type    | Use for                |
| ------- | ---------------------- |
| `bad`   | the wrong way, a risk  |
| `ok`    | the right way          |
| `info`  | neutral information    |
| `warn`  | a warning, a trade-off |
| `plain` | no emphasis            |

```plotum
section bad "Without cache"
  api -> db : every request

section ok "With cache"
  api2 [API]
  api2 -> redis : GET key
```

A section lasts until the next `section`. Indentation is only decorative.

## Notes

```plotum
section info "Kafka"
  k [Kafka | topic chat]
  note: every server reads the same topic
```

## Layout

| Option     | Effect                                      | Example   |
| ---------- | ------------------------------------------- | --------- |
| `wrap: N`  | snake layout: a new row every N nodes        | `wrap: 4` |
| `@col,row` | force a node to a grid position (on a node) | `@2,0`    |

```plotum
section info "Pipeline"
  wrap: 3
  a [Commit]
  b [Build]
  c [Test]
  d [Deploy]
  a -> b
  b -> c
  c -> d
```

With `wrap`, rows alternate direction (left to right, then right to left) so long
flows stay compact.

```plotum
client (Client) @0,0
server (Server) @2,0
```

## Modifiers

Modifiers go after a node or a link.

| Modifier    | On         | Effect                                           | Example               |
| ----------- | ---------- | ------------------------------------------------ | --------------------- |
| `{color}`   | node, link | named color, theme variable, hex code or gradient | `{brand}`, `{#22C55E}` |
| `@col,row`  | node       | forced grid position                             | `@2,0`                |
| `icon=name` | node       | built-in or theme icon                           | `icon=database`       |
| `step=N`    | node, link | appears at step N in presentation mode           | `step=3`              |

```plotum
db [PostgreSQL] icon=database {brand} step=2
api -> db {#22C55E} step=2 : SELECT
```

On a link, write modifiers **before** the `:` label: everything after `:` is label text.

## Theme

```plotum
theme:
  mode: dark
  font: jetbrains-mono
  colors:
    brand: #6D28D9
    accent: #EC4899 / #F472B6
    bg: #FFFFFF / #0B0B10
  gradients:
    hero: brand -> accent
  icons:
    kafka: https://example.com/kafka.svg
```

- `mode`: `light` or `dark`, the mode the diagram opens in. Default: `light`.
- `font`: one of the bundled fonts below. Default: `inter`.
- `colors`: your own variables (`brand`, `accent`, ...) and overrides of the base
  colors `bg`, `card` and `ink`. Give one color for both modes, or two separated
  by `/`: the light one first, then the dark one.
- `gradients`: `name: color1 -> color2`. A gradient built from variables follows
  the current mode.
- `icons`: `name: url`. The icon is downloaded once and stored as a data URI;
  nothing is stored on a server.

### Fonts

Fonts are bundled with Plotum, so a diagram looks the same in the browser and in
exports.

| Name             | Style                    |
| ---------------- | ------------------------ |
| `inter`          | sans-serif, the default  |
| `roboto`         | sans-serif               |
| `jetbrains-mono` | monospace, code-friendly |

## Steps and presentation

Every node or link with `step=N` appears at step N. Items without `step` are
visible from the start.

```plotum
user (User) step=1
api [API] step=2
user -> api step=2 : login
api -> db step=3 : check password
```

## Errors and warnings

- Every error has a position (line, column, length), says what was expected and
  what was found, and has a name, for example `UndefinedRelation`.
- The parser keeps going after an error, so all errors are reported at once.
- Warnings do not block rendering: node never linked, unknown color (default color
  used instead), icon not found.
