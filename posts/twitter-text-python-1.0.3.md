---
id: test-post
title: Release of twitter-text-python 1.0.3
description: I recently took over maintenance of this library and released version 1.0.3.
date: 2014-07-31T02:35:22Z
updated: 2014-07-31T02:35:22Z
draft: false
tags:
    - python
---

I recently took over maintenance of [twitter-text-python](https://pypi.python.org/pypi/twitter-text-python/), a Python
library which parses the content of Tweets, extracting URLs, usernames, and hashtags; and generates HTML for output and
display. Since the Twitter API returns plain statuses without any markup, you'll probably need to handle this in your
application - something that TTP makes incredibly easy.

To install:

```shell
pip install twitter-text-python
```

Usage example:

```python
>>> from ttp import ttp

>>> status = "@burnettedmond, go downstairs and throw a tarp" \
    "over anything that says #OperationHenessey on it." \
    "https://github.com/edburnett"

>>> p = ttp.Parser()
>>> result = p.parse(status)

>>> result.urls
['https://github.com/edburnett']

>>> result.users
['burnettedmond']

>>> result.tags
['OperationHenessey']

>>> result.html
u'<a href="https://twitter.com/burnettedmond">@burnettedmond</a>, go downstairs and throw a tarp over anything that says <a href="https://twitter.com/search?q=%23OperationHenessey">#OperationHenessey</a> on it. <a href=https://github.com/edburnett">https://github.com/edburnett</a>'
```

Today I'm releasing v1.0.3, which updates the generated URLs for compatibility with version 1.1 of the Twitter API. Some
other planned updates include Python 3 support, allowing the caller to specify a target for links (such as
`target="_blank"`), and a refresh of the documentation.

TTP is released under the MIT license. It was originally authored by [Ivo Wetzel](https://github.com/BonsaiDen), and was
previously maintained by [Ian Ozsvald](https://github.com/ianozsvald).

Check out the [project page](https://github.com/edmondburnett/twitter-text-python) on Github.

Update: Since the Twitter public API was effectively killed by the new ownership, and the paywall makes testing and
development impractical, this module was archived will no longer be actively maintained. It should work fine with any
existing API data.
