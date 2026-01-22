---
id: squid-custom-url-rewriting
title: Custom URL rewriting with Squid
description: How to perform arbitrary URL rewriting on a caching Squid proxy with a simple bit of Python.
date: 2015-11-07T19:09:40Z
updated: 2026-01-21T18:42:02Z
draft: false
tags:
    - caching
    - python
---

I recently ran into an issue where I needed to remove a port number from URLs forwarded from a Squid caching proxy
server. In my specific use case, the port number arrives from some custom handling being done for a client, but this
method can be used to redirect or rewrite URLs matching any criteria you specify.

For this purpose, Squid provides the [url_rewrite_program](http://www.squid-cache.org/Doc/config/url_rewrite_program/)
directive. This can point to an executable of any sort, such as a Ruby or shell script. In this case, I'm using Python.

```python
#!/usr/bin/python

import sys
import re

regex = re.compile(r':8082')

def rewrite_url(line):
    """ If the input URL contains a port number, strip it out """
    url_list = line.split(' ')
    old_url = url_list[0]
    new_url = '\n'
    if regex.search(old_url):
        new_url = re.sub(regex, '', old_url, count=1) + new_url
    return new_url

while True:
    line = sys.stdin.readline().strip()
    new_url = rewrite_url(line)
    sys.stdout.write(new_url)
    sys.stdout.flush()
```

View on [gist](https://gist.github.com/edmondburnett/e5d0a487e4a898931548).

Within the `while True:` block, we receive an input string from Squid via stdin. This string gets sent to our
rewrite_url function, which simply grabs the URL, tries to match our criteria using regex, and if a match is found, we
strip the matching string (in this case, a specific trailing port number) from the URL. The script then simply outputs
the new result on stdout, immediately flushing the stdout buffer.

Now, we just need to make this script executable, then edit our squid.conf and point it to our rewriter, and restart
Squid.

```squidconf
url_rewrite_program /etc/squid/custom_rewrite.py
url_rewrite_children 12
```

Here we also ask Squid to spawn 12 Python processes using the
[url_rewrite_children](http://www.squid-cache.org/Doc/config/url_rewrite_children/) directive. I was at one time using
this setup in a production application. Adjust as necessary for your environment.
