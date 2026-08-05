---
id: uxntal-syntax-plugin-neovim
title: Uxntal Syntax Highlighting Plugin for Neovim
description: Ported a vim syntax highlighter plugin for the Uxntal assembly language to Neovim
date: 2026-08-05T09:50:22Z
updated: 2026-08-05T10:35:34Z
draft: false
tags:
    - neovim
---

Yesterday I published [uxntal.nvim](https://github.com/edmondburnett/uxntal.nvim), a file detection and syntax plugin
for Neovim, written in Lua.

![Uxntal plugin screenshot](/static/images/uxntal-plugin.png "Uxntal displayed in Neovim. Colorscheme is Nightfox, font
is Jetbrains Mono.")

I had noticed that links to the original vimscript-based plugin were 404ing, thus breaking updates from my Neovim plugin
manager [lazy.nvim](https://github.com/folke/lazy.nvim). After some searching around with no luck, I checked for an
archived copy of the github repo via archive.org, and found it had been retired by the owner, whose account was no
longer public. I was then able to locate this IP-address-only self-hosted [repo](https://167.235.19.20/uxntal.vim/); it
seems the project has been moved off of Github.

It occurred to me that I should either mirror this repo in case it disappears again, or write my own version
specifically for Neovim. It didn't take long to port the plugin from Vimscript to Lua. For completeness, I also added
additional support for matchpairs (i.e. so that `%` jumps between pairs), `:checkhealth`, and vimdocs `:h uxntal`.

I occasionally enjoy experimenting with esolangs, and the [uxn](https://wiki.xxiivv.com/site/uxn.html) virtual machine
offers a fun ecosystem along with an interesting thought experiment: [permacomputing](https://permacomputing.net/); or
building software that stays useful for decades, is maintainable by a single person, small enough to run on old or
scavenged hardware, using limited power and compute resources, with no dependency on any particular platform or company
to stay alive.

A plugin's home disappearing because it lived on someone else's infrastructure could be seen as a fitting little case
study in why that appeal is more than aesthetic. For now, I am hosting my Lua version on
[github](https://github.com/edmondburnett/uxntal.nvim) and [codeberg](https://codeberg.org/edmondburnett/uxntal.nvim).
