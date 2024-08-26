# Typst Introduction

## markdown代码块测试
```cpp
代码块
```

&lt;!--more--&gt;
&lt;h1 align=&#34;center&#34;&gt;
  &lt;img alt=&#34;Typst&#34; src=&#34;https://user-images.githubusercontent.com/17899797/226108480-722b770e-6313-40d7-84f2-26bebb55a281.png&#34;&gt;
&lt;/h1&gt;

&lt;p align=&#34;center&#34;&gt;
  &lt;a href=&#34;https://typst.app/docs/&#34;&gt;
    &lt;img alt=&#34;Documentation&#34; src=&#34;https://img.shields.io/website?down_message=offline&amp;label=docs&amp;up_color=007aff&amp;up_message=online&amp;url=https%3A%2F%2Ftypst.app%2Fdocs&#34;
  /&gt;&lt;/a&gt;
  &lt;a href=&#34;https://typst.app/&#34;&gt;
    &lt;img alt=&#34;Typst App&#34; src=&#34;https://img.shields.io/website?down_message=offline&amp;label=typst.app&amp;up_color=239dad&amp;up_message=online&amp;url=https%3A%2F%2Ftypst.app&#34;
  /&gt;&lt;/a&gt;
  &lt;a href=&#34;https://discord.gg/2uDybryKPe&#34;&gt;
    &lt;img alt=&#34;Discord Server&#34; src=&#34;https://img.shields.io/discord/1054443721975922748?color=5865F2&amp;label=discord&amp;labelColor=555&#34;
  /&gt;&lt;/a&gt;
  &lt;a href=&#34;https://github.com/typst/typst/blob/main/LICENSE&#34;&gt;
    &lt;img alt=&#34;Apache-2 License&#34; src=&#34;https://img.shields.io/badge/license-Apache%202-brightgreen&#34;
  /&gt;&lt;/a&gt;
  &lt;a href=&#34;https://typst.app/jobs/&#34;&gt;
    &lt;img alt=&#34;Jobs at Typst&#34; src=&#34;https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Ftypst.app%2Fassets%2Fdata%2Fshields.json&amp;query=%24.jobs.text&amp;label=jobs&amp;color=%23A561FF&amp;cacheSeconds=1800&#34;
  /&gt;&lt;/a&gt;
&lt;/p&gt;

Typst is a new markup-based typesetting system that is designed to be as powerful
as LaTeX while being much easier to learn and use. Typst has:

- Built-in markup for the most common formatting tasks
- Flexible functions for everything else
- A tightly integrated scripting system
- Math typesetting, bibliography management, and more
- Fast compile times thanks to incremental compilation
- Friendly error messages in case something goes wrong

This repository contains the Typst compiler and its CLI, which is everything you
need to compile Typst documents locally. For the best writing experience,
consider signing up to our [collaborative online editor][app] for free. It is
currently in public beta.

## Example
A [gentle introduction][tutorial] to Typst is available in our documentation.
However, if you want to see the power of Typst encapsulated in one image, here
it is:
&lt;p align=&#34;center&#34;&gt;
 &lt;img alt=&#34;Example&#34; width=&#34;900&#34; src=&#34;https://user-images.githubusercontent.com/17899797/228031796-ced0e452-fcee-4ae9-92da-b9287764ff25.png&#34;/&gt;
&lt;/p&gt;


Let&#39;s dissect what&#39;s going on:

- We use _set rules_ to configure element properties like the size of pages or
  the numbering of headings. By setting the page height to `auto`, it scales to
  fit the content. Set rules accommodate the most common configurations. If you
  need full control, you can also use [show rules][show] to completely redefine
  the appearance of an element.

- We insert a heading with the `= Heading` syntax. One equals sign creates a top
  level heading, two create a subheading and so on. Typst has more lightweight
  markup like this, see the [syntax] reference for a full list.

- [Mathematical equations][math] are enclosed in dollar signs. By adding extra
  spaces around the contents of an equation, we can put it into a separate block.
  Multi-letter identifiers are interpreted as Typst definitions and functions
  unless put into quotes. This way, we don&#39;t need backslashes for things like
  `floor` and `sqrt`. And `phi.alt` applies the `alt` modifier to the `phi` to
  select a particular symbol variant.

- Now, we get to some [scripting]. To input code into a Typst document, we can
  write a hash followed by an expression. We define two variables and a
  recursive function to compute the n-th fibonacci number. Then, we display the
  results in a center-aligned table. The table function takes its cells
  row-by-row. Therefore, we first pass the formulas `$F_1$` to `$F_8$` and then
  the computed fibonacci numbers. We apply the spreading operator (`..`) to both
  because they are arrays and we want to pass the arrays&#39; items as individual
  arguments.

&lt;details&gt;
  &lt;summary&gt;Text version of the code example.&lt;/summary&gt;

  ```typst
  #set page(width: 10cm, height: auto)
  #set heading(numbering: &#34;1.&#34;)

  = Fibonacci sequence
  The Fibonacci sequence is defined through the
  recurrence relation $F_n = F_(n-1) &#43; F_(n-2)$.
  It can also be expressed in _closed form:_

  $ F_n = round(1 / sqrt(5) phi.alt^n), quad
    phi.alt = (1 &#43; sqrt(5)) / 2 $

  #let count = 8
  #let nums = range(1, count &#43; 1)
  #let fib(n) = (
    if n &lt;= 2 { 1 }
    else { fib(n - 1) &#43; fib(n - 2) }
  )

  The first #count numbers of the sequence are:

  #align(center, table(
    columns: count,
    ..nums.map(n =&gt; $F_#n$),
    ..nums.map(n =&gt; str(fib(n))),
  ))
  ```
&lt;/details&gt;

## Installation
Typst&#39;s CLI is available from different sources:

- You can get sources and pre-built binaries for the latest release of Typst
  from the [releases page][releases]. Download the archive for your platform and
  place it in a directory that is in your `PATH`. To stay up to date with future
  releases, you can simply run `typst update`.

- You can install Typst through different package managers. Note that the
  versions in the package managers might lag behind the latest release.
  - Linux: View [Typst on Repology][repology]
  - macOS: `brew install typst`
  - Windows: `winget install --id Typst.Typst`

- If you have a [Rust][rust] toolchain installed, you can install
  - the latest released Typst version with
    `cargo install --locked typst-cli`
  - a development version with
    `cargo install --git https://github.com/typst/typst --locked typst-cli`

- Nix users can
  - use the `typst` package with `nix-shell -p typst`
  - build and run a development version with
    `nix run github:typst/typst -- --version`.

- Docker users can run a prebuilt image with
  `docker run -it ghcr.io/typst/typst:latest`.

## Usage
Once you have installed Typst, you can use it like this:
```sh
# Creates `file.pdf` in working directory.
typst compile file.typ

# Creates PDF file at the desired path.
typst compile path/to/source.typ path/to/output.pdf
```

You can also watch source files and automatically recompile on changes. This is
faster than compiling from scratch each time because Typst has incremental
compilation.
```sh
# Watches source files and recompiles on changes.
typst watch file.typ
```

Typst further allows you to add custom font paths for your project and list all
of the fonts it discovered:
```sh
# Adds additional directories to search for fonts.
typst compile --font-path path/to/fonts file.typ

# Lists all of the discovered fonts in the system and the given directory.
typst fonts --font-path path/to/fonts

# Or via environment variable (Linux syntax).
TYPST_FONT_PATHS=path/to/fonts typst fonts
```

For other CLI subcommands and options, see below:
```sh
# Prints available subcommands and options.
typst help

# Prints detailed usage of a subcommand.
typst help watch
```

If you prefer an integrated IDE-like experience with autocompletion and instant
preview, you can also check out the [Typst web app][app], which is currently in
public beta.

## Community
The main place where the community gathers is our [Discord server][discord].
Feel free to join there to ask questions, help out others, share cool things
you created with Typst, or just to chat.

Aside from that there are a few places where you can find things built by
the community:

- The official [package list](https://typst.app/docs/packages)
- The [Awesome Typst](https://github.com/qjcg/awesome-typst) repository

If you had a bad experience in our community, please [reach out to us][contact].

## Contributing
We would love to see contributions from the community. If you experience bugs,
feel free to open an issue. If you would like to implement a new feature or bug
fix, please follow the steps outlined in the [contribution guide][contributing].

To build Typst yourself, first ensure that you have the
[latest stable Rust][rust] installed. Then, clone this repository and build the
CLI with the following commands:

```sh
git clone https://github.com/typst/typst
cd typst
cargo build --release
```

The optimized binary will be stored in `target/release/`.

Another good way to contribute is by [sharing packages][packages] with the
community.

## Pronunciation and Spelling
IPA: /taɪpst/. &#34;Ty&#34; like in **Ty**pesetting and &#34;pst&#34; like in Hi**pst**er. When
writing about Typst, capitalize its name as a proper noun, with a capital &#34;T&#34;.

## Design Principles
All of Typst has been designed with three key goals in mind: Power,
simplicity, and performance. We think it&#39;s time for a system that matches the
power of LaTeX, is easy to learn and use, all while being fast enough to realize
instant preview. To achieve these goals, we follow three core design principles:

- **Simplicity through Consistency:**
  If you know how to do one thing in Typst, you should be able to transfer that
  knowledge to other things. If there are multiple ways to do the same thing,
  one of them should be at a different level of abstraction than the other. E.g.
  it&#39;s okay that `= Introduction` and `#heading[Introduction]` do the same thing
  because the former is just syntax sugar for the latter.

- **Power through Composability:**
  There are two ways to make something flexible: Have a knob for everything or
  have a few knobs that you can combine in many ways. Typst is designed with the
  second way in mind. We provide systems that you can compose in ways we&#39;ve
  never even thought of. TeX is also in the second category, but it&#39;s a bit
  low-level and therefore people use LaTeX instead. But there, we don&#39;t really
  have that much composability. Instead, there&#39;s a package for everything
  (`\usepackage{knob}`).

- **Performance through Incrementality:**
  All Typst language features must accommodate for incremental compilation.
  Luckily we have [`comemo`], a system for incremental compilation which does
  most of the hard work in the background.

[docs]: https://typst.app/docs/
[app]: https://typst.app/
[discord]: https://discord.gg/2uDybryKPe
[tutorial]: https://typst.app/docs/tutorial/
[show]: https://typst.app/docs/reference/styling/#show-rules
[math]: https://typst.app/docs/reference/math/
[syntax]: https://typst.app/docs/reference/syntax/
[scripting]: https://typst.app/docs/reference/scripting/
[rust]: https://rustup.rs/
[releases]: https://github.com/typst/typst/releases/
[repology]: https://repology.org/project/typst/versions
[contact]: https://typst.app/contact
[architecture]: https://github.com/typst/typst/blob/main/docs/dev/architecture.md
[contributing]: https://github.com/typst/typst/blob/main/CONTRIBUTING.md
[packages]: https://github.com/typst/packages/
[`comemo`]: https://github.com/typst/comemo/

---

> 作者:   
> URL: http://localhost:1313/CTGU-Hugo-Blog.io/posts/0077ac4/  
> 转载 URL: https://github.com/typst/typst/blob/main/README.md?plain=1
