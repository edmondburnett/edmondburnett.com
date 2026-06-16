---
id: meta-ai-crawling-ddos
title: Meta DDoS'd my Mastodon instance
description: Handling a denial-of-service incident caused by Meta AI crawlers.
date: 2026-06-15T15:13:36Z
updated: 2026-06-15T15:13:40Z
draft: false
tags:
    - meta
    - facebook
    - infosec
---

A few days ago I received an alert from my host provider: The inbound traffic rate on one of my servers has been
exceeding alarm thresholds for several hours. Upon SSH'ing into the box, the system alerted that the disk was full.
Something aberrant was clearly happening.

According to `htop`, several processes were pegged at 100% usage. And as you have likely experienced, when disks fill
up, weird stuff can happen: some common troubleshooting commands fail to run, databases show log writing issues, etc. So
I set about cleaning up a few caches and local temp backups, while also checking and shutting down Postgres, Mastodon,
Elasticsearch, Redis, and a few other processes.

With a little headroom on the disk again, I could run `ncdu` and see that Mastodon's media cache had grown until it
consumed nearly the entire volume. When an instance encounters a remote post, it fetches and caches the attached media —
avatars, header images, photos, video — so it can render that content locally. Normally this cache stays bounded and
gets pruned on a schedule, but the flood of incoming requests was pulling down remote media far faster than any normal
cleanup routine could manage. Once the disk filled, Postgres could no longer write, Elasticsearch tipped over, and the
rest of the stack followed.

The bot was crawling public, federated content by way of my instance. By default, Mastodon will render public posts,
profiles, and hashtag timelines from any other server it has peered with. The crawler was working through paths like
these (anonymized):

```
GET /@anonymous@piaille.fr/113980271490580520
GET /@anonymous@pixelfed.art/114144537441489304
GET /api/v1/accounts/109303441654200732/statuses?exclude_replies=true
GET /api/v1/statuses/116469961523621540
GET /api/v1/tags/35mmportrait
GET /api/v1/tags/80sMovies
```

Each remote account or status is being proxied and rendered locally, and each render is an opportunity for my server to
go fetch and cache yet more remote media. The hashtag and `/api/v1/tags/` endpoints are even worse, since a single
popular tag fans out to posts from hundreds of other servers. To a crawler that follows every link it finds, one small
Mastodon instance becomes a doorway into the entire fediverse — and a remarkably efficient way to make someone else's
server do enormous amounts of expensive work on its behalf.

Some time ago, in anticipation of Meta linking their platforms with the Fediverse, many instances (including my own)
became signatories of [Fedipact](https://fedipact.online/), a pledge to broadly defederate Meta from the network. But
this method of harvesting bypasses the need to access content through normal federation.

Part of the fix moving forward was to turn this remote-browsing behavior off entirely. To do this, go to
Preferences->Administration->Server Settings->Discovery and turn the two remote options under the Public Timelines
section to `Authenticated Users Only`. Unfortunately, leaving this feature on is leaving an open invitation for the next
AI harvester to do exactly the same thing. For a small instance, trading a bit of that openness in exchange for a server
that stays online feels like the obvious call.

Outside of this, the softest approach is to disallow the `meta-externalagent` user agent in your `robots.txt`. Meta
[claims](https://developers.facebook.com/docs/sharing/webmasters/web-crawlers#identify-4) to honor this, but after a few
hours the requests were still coming. I suspect if you aren't prepared with this opt-out ahead of time, the bot simply
proceeds and they don't bother to keep checking for an updated `robots.txt`.

```robots_txt
User-agent: meta-externalagent
Disallow: /
```

The next step was to forbid the bot from accessing the site via the nginx config, by adding it to my list of unwanted
agents and returning a 403 Forbidden.

```
# block unwanted user agents (snipped)
if ($http_user_agent ~* (YandexBot|Yandex|ZyBorg|meta-externalagent)) {
    return 403;
}
```

Now, requests from the bot are 403'ing, as expected:

```nginx_log
2a03:2880:f806:20:: - - [18/May/2026:21:05:05 +0000] "GET /@anonymous@aus.social HTTP/2.0" 403 146 "-" "meta-externalagent/1.1 (+https://developers.facebook.com/docs/sharing/webmasters/crawler)"
57.141.6.17 - - [18/May/2026:21:05:12 +0000] "GET /@anonymous@sonomu.club/116166841041339741 HTTP/2.0" 403 146 "-" "meta-externalagent/1.1 (+https://developers.facebook.com/docs/sharing/webmasters/crawler)"
```

But since the bot was reducing the readability of the logs, and causing them to grow to hundreds of megabytes by
continuing to flood nginx with erroring requests, I ended up just blocking the IP ranges being used to query the server.
This of course cleaned up the log spam.

```iptables
-A INPUT -s 57.141.6.0/24 -j DROP
```

```ip6tables
-A INPUT -s 2a03:2880:f806::/48 -j DROP
```

Note that these IP ranges are likely regional and may be different for you. Another consideration is you also may want
to continue to allow [other Meta bots](https://developers.facebook.com/docs/sharing/webmasters/web-crawlers) to access
your application. FacebookExternalHit, for example, caches preview cards when someone posts a link to your site on a
Meta platform.

I suppose the lesson here is that the old default — leave things open in good faith, let the public web be public — is
largely unsustainable in the present technology landscape. None of the steps I took here were difficult, but closing off
public remote posts quietly subtracted something. The fediverse was merely working as intended: any server could be a
window onto any other. Closing that window protects my little instance, but it also makes the network feel a little less
connected in the public commons.

That same calculation is now playing out everywhere. Faced with predatory harvesting that treats every public endpoint
as free training data, most operators are doing exactly what I did, only at larger scale: gating content behind logins,
throwing up "are you human" interstitials, rate-limiting aggressively, or paywalling. Each of these is individually
reasonable. After all, we never accepted this sort of behaviour from spam bots. Collectively, they're remaking the web
into something far less open than it was intended to be — a place where a great deal of human creative output ends up
behind increasing levels of fortification, because the cost of leaving the door open is now too high.

The web was built on a kind of casual generosity — publish something, and anyone can read it — and that generosity is
precisely what is sadly being strip-mined, unlicensed, at industrial scale.
