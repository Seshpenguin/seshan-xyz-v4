---
title: We got an Atom/RSS feed! Who asked for this again?
author: Seshan Ravikumar
type: post
date: 2018-12-10T21:20:33+00:00
url: /2018/12/10/we-got-an-atom-rss-feed-who-asked-for-this-again/
categories:
  - Uncategorized

---
<blockquote class="wp-block-quote">
  <p>
    May 2019 Update: This blog post was on my old, custom blog system. Now that I got bored of it, we have a new blog systems.<br />Just keep in mind that this post no longer reflects how the blog works nowadays.
  </p>
</blockquote>

In the ever lasting quest of adding more random stuff to this website for our own entertainment, ladies and gentlemen I present to you the Atom feed for the blog!!!  
Ok so first thing I had to do was actually look up the Atom spec&#8230; I spent maybe 5 minutes staring at the RFC then just copied the example and went from there. Just FYI, there is no way this thing is standard compliant. It&#8217;s a miracle is this feed is useful to anyone lol.  
  
The actual implementation wasn&#8217;t that hard at all. First thing was in IIS 6 I had to associate the .xml extension with the asp.dll executable so IIS would run the embedded VBScript. <figure class="wp-block-image">

![][1] </figure> 

The code itself was pretty much copied from the main blog code (that is, load each blog and it&#8217;s title from numbered files on the drive), I just had to change it so it would generate &#8220;valid&#8221; XML instead of HTML. It&#8217;s mostly pretty sane, except my way of handling the HTML markup that&#8217;s included in posts&#8230;  
<figure class="wp-block-image">

![][2] </figure> 

Yep&#8230; It&#8217;s good enough so none of the HTML breaks the XML, but do expect some littering of leftover markup in the feed summaries. Obviously not perfect, but did you really expect reasonable things here?? Thought so! xD  
  
More importantly, [here is a link to the Atom feed.][3] Go nuts!

 [1]: http://legacy.seshan.xyz/flow/files/6-iis-screenshot.jpg
 [2]: http://legacy.seshan.xyz/flow/files/6-code-screenshot.jpg
 [3]: https://seshan.xyz/flow/feed/atom.xml