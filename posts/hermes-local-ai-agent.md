---
id: hermes-local-ai-agent
title: Running a 35-billion-parameter AI agent on an 8 GB Nvidia GPU
description: Setting up, integrating, and benchmarking Hermes, a self-hosted AI agent.
date: 2026-10-02T18:10:43Z
updated: 2026-10-02T18:10:43Z
draft: false
tags:
    - ai
---

Can a decade-old PC with an early-2020s GPU run a genuinely useful AI agent, using a free model?

My curiosity recently returned about the current state of self-hosted LLMs, and what the realistic use cases and
limitations might be on random commodity hardware. The machine I chose for this experiment is an older headless mini-ITX
Arch Linux box I've been using as a media server, which also hosts a few homelab tasks such as Pi-hole DNS, a k3s
Kubernetes node, and a very nerdy private IRC server (which I'm rather excited to write about sometime). It has an Intel
i7-6700K (from 2015), 32 GB of RAM, and an RTX 3060 Ti graphics card with 8 GB of VRAM. And for the last few days I've
been running [Hermes](https://hermes-agent.nousresearch.com/), an open-source AI agent from Nous Research, backed by
Qwen3.6, a 35-billion-parameter model that was a good fit for the available hardware.

![Writing speed 30–34 tokens per second, reading speed 300–375 tokens per second, about 80 seconds of warm-up on the first message, and 6.5 minutes for a real multi-step task.](/static/images/hermes-local-ai-agent/at-a-glance-crop.png)

It's a little faster than I expected, and much more performant than my earlier naive attempts at running local LLMs. The
model writes at 30–34 tokens per second, or roughly 22–25 words per second, which is quicker than many people read. The
slowest part is the first message of a conversation. The agent has a long set of built-in instructions (about 24,000
tokens) that it has to read before answering, which takes around 80 seconds. After that it's cached, and follow-up
replies begin much faster.

This model is built from many small specialist sub-networks, and only a few of them are active for each word it
produces. Some tuning was needed to fit the most critical parts onto the GPU's 8 GB of VRAM, while the rest sit in
ordinary system memory.

How much of the model sits on the GPU VRAM vs system RAM makes a measurable difference:

![Writing speed rises from 29.8 to 35.6 tokens per second as fewer expert layers stay in system RAM; at 28 the model no longer fits in 8 GB.](/static/images/hermes-local-ai-agent/gpu-offload-tuning-crop.png)

## Use cases

Hermes isn't just a chatbot. Nous Research describes it as a self-improving agent, amassing memories and a library of
skills over time, and learning from a profile it builds on your interactions. It can run arbitrary commands (sandboxed
or otherwise, depending on how you set it up), write code, browse the web, interface with MCP servers, or act as an MCP
server itself.

One simple task I ran was to fetch my website, pull the colour scheme out of its stylesheet, and generate an image of
the palette. It worked through the problem in 17 steps and finished in about six and a half minutes.

![Where 388 seconds went: 79 seconds of warm-up, 64 seconds reading new information, 205 seconds writing, 40 seconds running tools.](/static/images/hermes-local-ai-agent/where-the-time-went-crop.png)

Longer conversations are also reasonably fast. With 32,000 tokens of history, around where the agent starts summarising
older messages to save space, it's only about 10% slower than with an empty conversation.

![Writing speed falls from 34.5 to 31.0 tokens per second, and reading speed from 387 to 353, as the conversation grows from empty to 32,000 tokens.](/static/images/hermes-local-ai-agent/speed-vs-context-length-crop.png)

These sorts of benchmarks begin to inform the potential use-cases. It is, of course, slower than cloud-based frontier
agents. My vision for it lies in less-complex duties: task & issue management for my GitHub projects, a conversational
Slack/Discord/IRC assistant, offloading simple tasks from paid models, an MCP frontend to my personal
note-taking/knowledge base system, one of several agents in a [Buzz workspace](https://github.com/block/buzz), or
perhaps running abliterated models for red-teaming or legitimate security research.

On Discord, it got some scripting use straight away. My girlfriend asked it to build a Python web crawler that finds
climate-related job listings and emails her a weekly digest. It wrote the script and then improved it over several
rounds of feedback, using a combination of a headless browser and public APIs to harvest data from various job
aggregators.

Around the same time, the server began logging memory-pressure warnings, which led me to a configuration fix (more on
that below). It also insisted it couldn't attach code files in Discord, and declined repeated requests to try. The cause
turned out to be its own memory: during early testing it had saved a note telling itself to put files in a sandbox
folder whose path Hermes couldn't attach from, and it kept following that note over my instructions. Its memory is a
plain markdown file, so the fix was simply to delete the offending lines. Being able to read and edit what the agent
believes about itself is one of the nicer parts of running it. For sharing larger bits of code, I also gave it access to
GitHub for Gists, along with a shared private repo.

## Installation & Setup

The rest of this post will dive into the technical details, for anyone who wants to reproduce something similar. There
are two main pieces: `llama.cpp` runs the model, while `Hermes` talks to it as the custom provider. Both run as systemd
services. I also downloaded the model's vision projector `mmproj` so that the agent can read images.

```shell
sudo pacman -S llama-cpp ggml-cuda
sudo mkdir -p /srv/models
sudo hf download unsloth/Qwen3.6-35B-A3B-GGUF --include '*UD-Q4_K_M*' --local-dir /srv/models
sudo hf download unsloth/Qwen3.6-35B-A3B-GGUF --include 'mmproj-F16.gguf' --local-dir /srv/models
sudo chmod -R a+rX /srv/models
```

Installing Hermes is pretty straightforward. `hermes model` opens a wizard where I set up a Custom Endpoint to
`http://127.0.0.1:8080/v1`, no key, `Chat Completions`, context `65536`, and reasoning set to `low` by default. I
skipped the [Nous Portal](https://portal.nousresearch.com/) model selection for my immediate purposes (those are
optional free and paid cloud models, feel free to explore these if you desire).

```shell
curl -fsSL https://hermes-agent.nousresearch.com/install.sh | bash
hermes model
hermes config set terminal.backend docker
hermes config set terminal.docker_image local/hermes-sandbox:pdf  # we create this later
hermes config set model.supports_vision true
```

### Choosing the model

The model is Qwen3.6-35B-A3B, quantised to 4 bits (Unsloth's `UD-Q4_K_M`, 20.6 GB). The "A3B" is the important part:
it's a mixture-of-experts model, so although it has 35 billion parameters in total, only about 3 billion are active for
each token. That's what makes it usable on this hardware. A dense 35B model would need to read all of its weights for
every token, which from system RAM would be painfully slow. Here, each token only touches a small slice of the expert
weights, so those can live in RAM while the parts used on every token stay on the GPU.

### Serving it with llama.cpp

Arch Linux packages `llama-server` with its own systemd unit, so I overrode its command line with a drop-in:

`sudo systemctl edit llama-server`

```ini
[Service]
ExecStart=
ExecStart=/usr/bin/llama-server -m /srv/models/Qwen3.6-35B-A3B-UD-Q4_K_M.gguf \
  -np 1 -c 65536 -ngl 99 --n-cpu-moe 32 \
  --cache-type-k q8_0 --cache-type-v q8_0 -fa on \
  --jinja --host 127.0.0.1 --port 8080 --alias qwen3.6-35b-a3b \
  --mmproj /srv/models/mmproj-F16.gguf --no-mmproj-offload
Restart=on-failure
RestartSec=5
```

Then enable and start with `sudo systemctl enable --now llama-server` or `sudo systemctl restart llama-server` if it's
already running.

What the flags do:

- `-ngl 99 --n-cpu-moe 32` puts every layer on the GPU, then moves the expert weights of 32 layers back to system RAM.
  This is the main tuning knob, covered below.
- `-c 65536` gives a 64K-token context window. Hermes needs a large one: its system prompt alone is about 24K tokens,
  and it starts compressing history at half the window.
- `-np 1` runs a single slot. With several parallel slots the context is split between them, and the extra per-slot
  state was enough to run out of VRAM on first start. One agent only needs one slot.
- `--cache-type-k q8_0 --cache-type-v q8_0 -fa on` stores the KV cache at 8 bits instead of 16, roughly halving its
  memory with negligible quality loss. A quantised V cache requires flash attention.
- `--jinja` uses the jinja chat template embedded in the model file, which is what makes tool calling work.
- The `--mmproj` line loads the vision projector, so the agent can read images, and keeps it on the CPU to save VRAM.
- `--host 127.0.0.1` binds the API to localhost.

### Tuning the GPU split

How many expert layers to keep in RAM was the one setting worth benchmarking. After stopping `llama-server`, I ran
`llama-bench` at several `-ncmoe` values, starting high at `40` and working down until it no longer fit at `28`:

```shell
llama-bench -m /srv/models/Qwen3.6-35B-A3B-UD-Q4_K_M.gguf \
  -ngl 99 -ncmoe 40 -fa 1 -ctk q8_0 -ctv q8_0 -p 512,4096 -n 128
```

40, 36, 32 and 30 ran; 28 ran out of VRAM. Writing speed rose from 29.8 to 35.6 tokens per second as more experts moved
onto the GPU (see the chart earlier in this post). I settled on using `-ncmoe 32` in the systemd unit rather than the
faster 30, because the benchmark doesn't account for a full 64K context and the vision projector, and a server that
falls over under load is worse than one that's 3% slower.

### The memory-pressure fix

Early on, I had used `--load-mode none` to stop llama.cpp memory-mapping the model file, which copies the weights into
the process's own memory instead. On a 32 GB machine with a 20 GB model, that left about 6 GB for everything else, and
the server began swapping and logging memory-pressure warnings.

![Removing --load-mode none raised available RAM from 6.1 to 27 GiB and cut model load time from about 60 seconds to 2.6, with no measurable change in writing speed.](/static/images/hermes-local-ai-agent/mmap-memory-fix-crop.png)

Dropping the flag fixed it. With the default `mmap`, the weights in system RAM are backed by the file on disk and held
in the page cache. They're still in memory, but they're clean pages that the kernel can reclaim and re-read rather than
swap out, so they no longer count against everything else on the box. The 27 GiB "available" figure overstates the real
headroom for that reason: if other programs did claim that memory, the model would slow down while it re-read its
weights from disk. In practice it hasn't done that (yet), and writing speed was unchanged (33.7 tokens per second in
use, against 34.4 benchmarked). The model also loads in 2.6 seconds instead of a minute.

### Hermes

Hermes connects to `llama-server` as a custom provider. The setup wizard wrote most of this in `~/.hermes/config.yaml`.
The `context_length` setting matters, because it tells Hermes how large the window actually is:

```yaml
model:
    default: "qwen3.6-35b-a3b"
    provider: "custom"
    base_url: "http://127.0.0.1:8080/v1"
    api_mode: chat_completions
    supports_vision: true

custom_providers:
    - name: Local (127.0.0.1:8080)
      base_url: http://127.0.0.1:8080/v1
      model: qwen3.6-35b-a3b
      api_mode: chat_completions
      models:
          qwen3.6-35b-a3b:
              context_length: 65536
```

### The Docker sandbox

Hermes can run commands directly on the host, but for obvious reasons, I want its work to be confined to a container. It
supports a Docker backend, so I built a custom image on top of Nous Research's sandbox, adding tools for documents,
media, scripting and GitHub. I created `~/hermes-sandbox/Dockerfile`:

```dockerfile
# ~/hermes-sandbox/Dockerfile
FROM nousresearch/hermes-sandbox:desktop
ENV PIP_ROOT_USER_ACTION=ignore
RUN apt-get update && apt-get install -y --no-install-recommends \
      git curl jq ripgrep fd-find file zip unzip xz-utils tree wget fzf less \
      poppler-utils pandoc tesseract-ocr ocrmypdf \
      ffmpeg sox imagemagick sqlite3 \
      build-essential lua5.4 shellcheck \
      dnsutils iputils-ping netcat-openbsd openssl \
 && rm -rf /var/lib/apt/lists/* \
 && pip install --no-cache-dir pypdf pandas pyyaml requests beautifulsoup4

# github cli integration
RUN mkdir -p -m 755 /etc/apt/keyrings \
 && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg -o /etc/apt/keyrings/githubcli-archive-keyring.gpg \
 && chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
 && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" > /etc/apt/sources.list.d/github-cli.list \
 && apt-get update && apt-get install -y --no-install-recommends gh \
 && rm -rf /var/lib/apt/lists/* \
 && git config --system user.name "<name>'s Hermes" \
 && git config --system user.email "<github email address>" \
 && git config --system credential.https://github.com.helper '!gh auth git-credential'
```

This is built by running `docker build -t local/hermes-sandbox:pdf ~/hermes-sandbox`, and wired up in
`~/.hermes/config.yaml`:

```yaml
terminal:
    backend: "docker"
    docker_image: local/hermes-sandbox:pdf
    docker_volumes:
        # same path inside and out, so the agent's MEDIA: attachment paths resolve
        - "/home/edmond/.hermes/cache/documents:/home/edmond/.hermes/cache/documents"
    docker_forward_env: ["GH_TOKEN"]
    container_cpu: 1
    container_memory: 5120 # MB
    container_disk: 51200 # MB
    container_persistent: true
```

With `container_persistent: true`, note that changes to docker volumes only take effect after you `docker rm -f` on the
old container.

### GitHub

I created a fine-grained access token on GitHub, limiting the agent to Gists and a few select repositories. Hermes
supplies the token into the sandbox as `GH_TOKEN` (the `docker_forward_env` line above), where the `gh` CLI picks it up,
and the git credential helper in the Dockerfile lets `git push` use it too. Its commits are authored as a separate
identity, so they're easy to tell apart from mine.

Set the token with `hermes config set GH_TOKEN <github_token>`, which gets stored in `~/.hermes/.env`.

### Discord integration

In the [Discord developer portal](https://discord.com/developers/applications), I created a new Discord application. On
the Installation page, set the Install Link to `None`. On the Bot page, enable `Message Content Intent` and
`Server Members Intent`, and turn `Public Bot` off. Save and hit `Reset Token` to get the application token. You'll also
need the application ID from the General Information page.

Discord user and channel access is restricted in Hermes's `.env` with `DISCORD_ALLOWED_USERS` and
`DISCORD_ALLOWED_CHANNELS`, and `DISCORD_FREE_RESPONSE_CHANNELS` lets it reply in some channels without being
@mentioned. In the Discord client, you can right-click on user and channel names to get the IDs, listing them
(comma-separated) in the appropriate environment vars. The application token gets assigned to `DISCORD_BOT_TOKEN`.

You then invite the bot to your server with
`https://discord.com/oauth2/authorize?client_id=<APP_ID>&scope=bot+applications.commands&permissions=309238025280`,
replacing `<APP_ID>` with the one you copied.

```shell
hermes gateway install
sudo loginctl enable-linger $USER
hermes gateway restart
journalctl --user -u hermes-gateway -f
```

Perhaps it's obvious, but just be aware that when Discord or some other commercial app is the interface, your
conversations pass through their servers. The model runs locally, but you do lose privacy for those sessions. For
anything sensitive, use the terminal interface instead, or perhaps some other secure chat integration (e.g. a private
IRC).

### Terminal access from other machines

The excellent Hermes agent TUI runs on the server. I use it from my workstation and laptop over SSH. In `~/.ssh/config`
I added:

```sshconfig
Host hermes
    HostName shade
    RequestTTY yes
    RemoteCommand ~/.local/bin/hermes --tui

Host hermes-tmux
    HostName shade
    RequestTTY yes
    RemoteCommand tmux -u new -A -s hermes '~/.local/bin/hermes --tui'

Host hermes-zellij
    HostName shade
    RequestTTY yes
    RemoteCommand zellij attach --create hermes options --default-layout hermes
```

`ssh hermes` opens a fresh session, while `ssh hermes-tmux` and `ssh hermes-zellij` variants attach to a persistent
session that survives disconnects. tmux needs `-u` to render the TUI's Unicode characters properly. The zellij variant
uses a custom layout that I created at `~/.config/zellij/layouts/hermes.kdl` on the server:

```kdl
layout {
    pane command="zsh" {
        args "-lc" "hermes --tui"
        close_on_exit true
    }
    pane size=1 borderless=true {
        plugin location="compact-bar"
    }
}
```

### Useful commands

```shell
hermes gateway restart  # after config changes
hermes config set <key> <value>
hermes mcp  # manage MCP servers
hermes lsp status

journalctl --user -u hermes-gateway -f
journalctl -u llama-server -f
```

## Closing thoughts

Along with my machine learning studies, having a real system to poke at makes the theory a little more concrete. Running
and tuning my own local model and agent inspires all sorts of fun integration possibilities that will no doubt carry
over to my other work. I also ended up learning more about how to choose a model, mixture-of-experts architectures, chat
templates, attention variants, and expert routing and quantisation.
