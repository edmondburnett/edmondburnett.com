---
id: meta-ai-crawling-ddos
title: DDoS'd by Meta
description: Handling a denial-of-service incident caused by Meta AI crawlers.
date: 2026-05-21T12:46:17Z
updated: 2026-05-21T12:46:19Z
draft: false
tags:
    - meta
    - facebook
    - infosec
---

A few days ago I received an alert from my host provider: The inbound traffic rate on one of my personal servers has
been exceeding alarm thresholds for several hours. Upon SSH'ing into the box, the system alerted that the disk was full.
Okay, this is a rather familiar moment. Something aberrant is happening.

Was I hacked? This is often the first anxious response my mind usually jumps to, although it rarely ends up being that
in reality. I set such thoughts aside.

According to htop, several processes were pegged at 100% usage. And as you have likely experienced, when disks fill up,
weird stuff can happen: some common troubleshooting commands fail to run, databases show log writing issues, etc. So I
set about cleaning up a few caches and local temp backups, while also checking and shutting down Postgres, Mastodon,
Elasticsearch, Redis, and a few other processes.
