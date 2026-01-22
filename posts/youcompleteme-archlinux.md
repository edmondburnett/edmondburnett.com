---
id: youcompleteme-archlinux
title: Vim C++ auto-completion with YCM on Archlinux
description: Getting the YouCompleteMe code-completion engine working on Arch.
date: 2014-10-20T05:58:02Z
updated: 2014-10-20T05:58:02Z
draft: false
tags:
    - vim
---

I've been doing more C++ development lately, and decided to check out the available code hinting solutions for the
language in Vim. The interwebs quickly suggested the [YouCompleteMe](https://github.com/ycm-core/YouCompleteMe) plugin
as the thing to try -- however, I hit a few temporary snags during the installation process. While the official docs
include instructions for Ubuntu, performing the same steps on Archlinux might leave you scanning error logs and Googling
for solutions.

YCM uses clang for native C/C++ code completion, and the plugin includes an installation script which builds & installs
a copy under it's own subdirectory. To avoid some dependency issues (which for me involved a missing libedit.so.2),
we're just going to use Archlinux's clang package from the official repos instead.

If you don't already have clang installed, let's take care of that now:

```shell
sudo pacman -Sy clang
```

Download and install the plugin itself. I use Vundle, which makes this as a simple as adding a line to your .vimrc:

```vim
Plugin 'valloric/YouCompleteMe'
```

Open Vim, and update/install the plugin:

```vim
:PluginUpdate
```

Now exit vim and run YCM's installation script, telling it to use the system clang:

```shell
cd ~/.vim/bundle/YouCompleteMe
./install --clang-completer --system-libclang
```

YCM should now be ready to use. For C/C++/Obj-C projects, don't forget to set your compiler flags in a
.ycm_extra_conf.py file in the project's home directory (refer the
[official docs](https://github.com/ycm-core/YouCompleteMe) for more details on usage).

