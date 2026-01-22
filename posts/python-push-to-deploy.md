---
id: python-push-to-deploy
title: Push-To-Deploy with Git, Python edition
description: Example of using a Python script for simple application deployment via Git hooks.
date: 2014-07-28T04:44:33Z
updated: 2026-01-21T18:33:38Z
draft: false
tags:
    - git
    - python
---

I have read and used [Kris Jordan's excellent guide](http://krisjordan.com/essays/setting-up-push-to-deploy-with-git) on
implementing git hooks to deploy code to a production server. I highly recommend following his tutorial if you're
interested in using git to deploy your application.

There's only one problem: The post-receive script is written in Ruby.

Okay, I guess that's not actually a problem. The Ruby script works just fine, although I did of course have to install
Ruby first to get it to run. However, for those of us who live in the Python universe, and may not be accustomed to
working with Ruby (or have it installed), I thought it might be handy to have a Python version of this script to work
off of.

```python
#!/usr/bin/env python
# post-receive hook for git-based deployments

import sys
import os
from subprocess import call

# configuration
deploy_to_path = '/path/to/deploy/directory/'
deploy_branch  = 'main'

def post_receive(from_commit, to_commit, branch_name):
    # Don't deploy if pushed branch != deploy_branch
    if not branch_name.endswith(deploy_branch):
        print('Received branch ' + branch_name
            + ', not deploying.')
        sys.exit()

    # copy files to deploy directory
    call('GIT_WORK_TREE="' + deploy_to_path
        + '" git checkout -f ' + branch_name, shell=True)
    print('DEPLOY: ' + branch_name + '(' + to_commit
        + ') copied to ' + deploy_to_path)

    # TODO: Deployment Tasks
    # i.e. Run a script, restart daemons, etc


if __name__ == '__main__':
    # get values from STDIN
    fc,tc,bn = sys.stdin.read().split()
    post_receive(fc, tc, bn)
```

Just name this script `post-receive` and place it in your bare remote repository under the `hooks` subfolder. Set
`deploy_to_path` and `deploy_branch` to appropriate values, and insert any additional custom tasks to be run at the end
of the `post_receive()` function. Everything else in Kris' tutorial still applies.

Be sure to make the script executable:

```shell
chmod +x post-receive
```

There is a [gist](https://gist.github.com/edmondburnett/40e7db34416fdc734846) of this script available.
