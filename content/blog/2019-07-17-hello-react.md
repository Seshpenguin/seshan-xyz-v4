---
title: Hello, React!
author: Seshan Ravikumar
type: post
date: 2019-07-17T19:37:41+00:00
url: /2019/07/17/hello-react/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
As many of you know, I&#8217;ll never be able to leave my website alone for very long&#8230; after all, I wanted it to be a representation of _me,_ and so it has to evolve with what I think is cool!

<blockquote class="wp-block-quote">
  <p>
    Before we go any further: Don&#8217;t worry, the current &#8220;<a href="https://github.com/Seshpenguin/xyzwp/">xyzwp</a>&#8221; website will stay as the default. I don&#8217;t want to regress any website features!
  </p>
</blockquote>

With that out of the way&#8230; [It&#8217;s a new website][1]! And it&#8217;s built in React! ?

I&#8217;ve recently been really impressed by React (and generally JS in 2019, blog post about that coming soon). So, I decided to try and create an alternative frontend for Seshan.XYZ using React & React Router.

One of the coolest parts is the new React-based site still has all the content from the regular website. Basically I am pulling in content from WordPress using their JSON REST API (which, if you ever wanted to pull data from my website for some reason, [is available here][2]). This is actually a pretty cool use of WordPress, as a so called &#8220;headless&#8221; CMS.

Anyway, feel free to explore the React site! As of right now it still lacks some features, like the comments box, for example. It&#8217;s also worth mentioning you&#8217;ll need a pretty modern browser to use it.

<div class="wp-block-button aligncenter">
  <a class="wp-block-button__link" href="https://seshan.xyz/react/">React Seshan.XYZ</a>
</div>

<hr class="wp-block-separator" />

Speaking of modern browsers, one of the goals for the current &#8220;xyzwp&#8221; website is that it should be at least be _usable_ with really old browsers. The oldest browser I&#8217;ve tested is IE6. As expected, CSS doesn&#8217;t work at all, but at least links still work, and you can actually find and read blog posts. 

I&#8217;ve noticed recently that the website is forced to use HTTPS when using the &#8220;seshan.xyz&#8221; domain (the .onion domain is fine). In order to keep old browser support going, you can use <http://legacy.seshan.xyz> to access the website without HTTPS. This works out nicely actually, since _most_ people who visit using the regular domain probably want HTTPS by default.

<hr class="wp-block-separator" />

Anyway, that&#8217;s it for now!

 [1]: https://seshan.xyz/react/
 [2]: https://seshan.xyz/wp-json