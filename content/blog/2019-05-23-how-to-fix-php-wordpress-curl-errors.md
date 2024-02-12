---
title: How to Fix PHP/WordPress Curl Errors
author: Seshan Ravikumar
type: post
date: 2019-05-23T16:39:39+00:00
url: /2019/05/23/how-to-fix-php-wordpress-curl-errors/
categories:
  - Uncategorized

---
Checking out that new super fancy Site Health tab of your WordPress Site, but notice that the REST API and Loopback checks are failing? How do you fix the Curl issue?

**Well, I have the solution (maybe)**! This probably only makes sense if you are hosting WordPress yourself instead of on a VPS or other commercial server, because this issue seems to affect servers that are behind a NAT.

**The Fix:** Add your domain name into &#8220;/etc/hosts&#8221;, mapped to 127.0.0.1 or your reverse proxy IP.

Why does this work? Basically whats happening is when WordPress is trying to communicate with itself using Curl, it uses the servers domain name. This is OK for servers that are not behind a NAT, because their Public IP can resolve locally, but is problematic when a server is behind a router, since you typically can&#8217;t just use your public IP. Adding a host entry forces WordPress to resolve your domain to localhost (or your reverse proxy) instead of the public IP, so it can talk to itself properly.