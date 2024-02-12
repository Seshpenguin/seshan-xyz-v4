---
title: Make the YouTube TV WebApp (leanback) Work Again!
author: Seshan Ravikumar
type: post
date: 2020-01-14T19:52:40+00:00
url: /2020/01/14/make-the-youtube-tv-webapp-leanback-work-again/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
 <figure class="wp-block-image size-large"><img loading="lazy" width="1024" height="595" src="https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-1024x595.jpg" alt="" class="wp-image-268" srcset="https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-1024x595.jpg 1024w, https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-300x174.jpg 300w, https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-768x446.jpg 768w, https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-1536x892.jpg 1536w, https://seshan.xyz/wp-content/uploads/2020/01/Screenshot_2020-01-14-YouTube-on-TV-2048x1190.jpg 2048w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure> 

For those running HTPC, a lot of you probably know about the [youtube.com/tv][1] Web UI (also known as Leanback). If you don&#8217;t know, Leanback is a fully featured YouTube experience designed for&#8230; well TVs and Remotes. It also bundles other nice TV features like the ability to &#8220;cast&#8221; to the TV. _And all it needs is a browsers!_

Sadly it appears Google is not longer letting normal browsers access YouTube TV&#8230; but there is a solution! As it turns out, some Smart TVs and Set Top Boxes use the web UI to provide YouTube, and Google didn&#8217;t want to break that. **So all you need to do is set your user agent to make YouTube TV/Leanback think you&#8217;re a Smart TV:**

<pre class="wp-block-code"><code>Mozilla/5.0 (SMART-TV; Linux; Tizen 4.0.0.2) AppleWebkit/605.1.15 (KHTML, like Gecko) SamsungBrowser/9.2 TV Safari/605.1.15</code></pre>

This will make your browser look like a Samsung Smart TV running Tizen!

In order to set the user agent, you&#8217;ll need some browser extension, [like this one for Firefox][2] (just create a custom entry for Youtube TV).

 [1]: http://youtube.com/tv
 [2]: https://addons.mozilla.org/en-CA/firefox/addon/uaswitcher/