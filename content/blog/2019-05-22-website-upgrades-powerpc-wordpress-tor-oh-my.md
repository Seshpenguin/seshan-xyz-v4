---
title: Website Upgrades! (PowerPC, WordPress, Tor, oh my!)
author: Seshan Ravikumar
type: post
date: 2019-05-22T21:25:06+00:00
url: /2019/05/22/website-upgrades-powerpc-wordpress-tor-oh-my/
categories:
  - Uncategorized

---
_&#8220;upgrades&#8221;_

Hey Guys! We got a new website! ? There are a couple of notable things about this brand new, amazing, awesome, website:

<ul class="browser-default">
  <li>
    Powered by WordPress (Ok that&#8217;s not too interesting but bear with me here&#8230;)
  </li>
  <li>
    A custom WordPress theme using Materialize (See, it&#8217;s not that bad!)
  </li>
  <li>
    Running on Ubuntu 16.04 (Again, not too interesting, but wait, there more!)
  </li>
  <li>
    &#8230; all on a Power Mac G5 Dual Processor (Aw yea! It&#8217;s pretty awesome).
  </li>
</ul>

I&#8217;ve just finished migrating all the old posts to this new website. It was pretty simple stuff, literally copy and paste the post from the browser.

One _slight_ issue I ran into was the images were pointing to the IP of the old AWS host. This is fine as long as the AWS server is online, but that won&#8217;t be long. My &#8220;solution&#8221; was to create a subdomain that temporarily points to the AWS server (legacy.seshan.xyz), so all the images keep that URL. Then I can point that legacy subdomain to this server and the images will still be served correctly.

Another interesting this about this website (and server) is Tor! I&#8217;ve been intrigued by Tor, and think it&#8217;s a really cool concept (If you aren&#8217;t familiar with it, it&#8217;s basically a super secure, super cool &#8220;vpn&#8221;. I&#8217;d recommend looking into it). If you look at the footer of this page you&#8217;ll see that this website does in fact have a .onion domain too, so you can securely and privately view my website!

Another thing related to Tor I am currently looking into is running a Relay on this server. Basically one of the concepts of Tor is you data is encrypted and then bounced between a bunch of relays to increase security. Running a relay is an easy way to help make the Tor network faster, more stable, and more secure. It&#8217;s also a lot less work that running something like an Exit Node (basically the server where your Tor request connects to the outside world if you browse a normal, not onion, website). I wonder how many Tor relays are running on PowerPC machines&#8230;

Anyway, that&#8217;s it for now. I hope you guys enjoy (or hate) the new website!