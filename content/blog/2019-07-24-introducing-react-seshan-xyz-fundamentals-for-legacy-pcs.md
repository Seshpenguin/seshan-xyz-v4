---
title: 'Introducing: React Seshan.XYZ – Fundamentals for Legacy PCs!'
author: Seshan Ravikumar
type: post
date: 2019-07-24T16:00:13+00:00
url: /2019/07/24/introducing-react-seshan-xyz-fundamentals-for-legacy-pcs/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
Have you ever wanted to browse my website, but were stuck with Netscape Navigator, IE 5, or something even more obscure? Well, I have the solution for you!

<blockquote class="wp-block-quote">
  <p>
    No Seshan, no one even visits your website.
  </p>
</blockquote>

On top of the regular xyzwp WordPress theme, and the recently introduced client-side React frontend, we have a third option to view this website: React Seshan.XYZ &#8211; Fundamentals for Legacy PCs (RSXYZFLP)!

<blockquote class="wp-block-quote">
  <p>
    Please just stop&#8230;
  </p>
</blockquote>

<div class="wp-block-button aligncenter">
  <a class="wp-block-button__link has-text-color has-very-dark-gray-color has-background has-pale-pink-background-color" href="http://flp.seshan.xyz/">React Seshan.XYZ &#8211; Fundamentals for Legacy PCs!</a>
</div>

RSXYZFLP has a number of key features. It no client-side JS, and is based around plain HTML to support **all the browsers!**

Here&#8217;s how it works behind the scenes: The website itself is written in React (plus React Router), but instead of letting React run and render on the client side, as it is designed to do, we instead make use of ReactDOMServer to render the app in a NodeJS Express app, which then serves the resulting HTML.

There are a few limitations of ReactDOMServer I had to work around, though. Most notably is that the `renderToStaticMarkup()` function is completely synchronous. Basically, I can&#8217;t actually fetch data from within the React part of the server.

To overcome this, I implemented a data caching system. When the Node app first starts, we call a function to load fresh data (page content, blog posts, etc,) into an object. Then, every request calls a function that will:

  * Serve cached content, and increment a freshness counter
  * If the freshness counter reaches a certain maximum, serve cached content and freshen the cache in the background.

This actually works out pretty well, since there is always data ready to be served, and the data object will be periodically updated. The reason we have to pull in all the data at once is because React Router handles the&#8230; well routing, the app doesn&#8217;t know what page is to be served until we are already executing renderToStaticMarkup (which calls React). Recall that React won&#8217;t be able to wait for async calls in this configuration, so the data has to be available before it starts rendering.

The way we pull in data is more or less the same function from React Seshan.XYZ, which uses the Axios library for really easy calling of the WordPress wp-json REST API.

_But wait! There&#8217;s more!_

I decided, just for the heck of it, let&#8217;s throw Espi.Dev in there too! His website makes use of lots of CSS3 things, like animations, which poor IE wouldn&#8217;t be able to keep up with.

To pull content from Espi.Dev, I used axios as usual, but this time we are just pulling in the straight HTML from Espidev&#8217;s website. In order to parse the HTML and pull out the data we need, I used Cheerio, a NodeJS implementation of jQuery that allows us to parse the html into a DOM, so we can pick out the content we need.

The NodeJS server itself is running on the same machine as everything else, a PowerMac G5 running Ubuntu 16.04. Now, while this _is_ a ppc64 (Big Endian) machine, and there are ppc64 builds of Node, I couldn&#8217;t get them to work. I think it has to do with that fact that Ubuntu uses a ppc64 kernel, but the userspace is just the regular 32-bit powerpc.

Since I couldn&#8217;t get Node 8 (there weren&#8217;t even ppc64 builds of Node 10, though ppc64le builds exist), the next best option was the Node 4, available from the package manager. Even though Node 4 is alarmingly old (and probably a really bad idea to run from a security standpoint), that didn&#8217;t stop me. What would stop me is if I had to write old-school JS that worked on Node 4. Thankfully, Babel to the rescue! Babel lets me write in modern ES2019, and have it &#8220;transpile&#8221; to old-school JS, bundling the necessary polyfills too!

So yea&#8230; as you can tell I am really bored and decided to write some more garbage code 😀

<div class="wp-block-button aligncenter">
  <a class="wp-block-button__link has-text-color has-very-dark-gray-color has-background has-pale-cyan-blue-background-color" href="https://github.com/Seshpenguin/react-seshanxyz/tree/master/react-seshanxyz-flp">View the code on GitHub</a>
</div>

Oh and if you were wondering about the name: It is a reference to [Windows Fundamentals for Legacy PCs][1], and version of Windows XP Embedded that Microsoft released to transition old systems running Windows NT 4, 95 etc to a more modern version of Windows.

 [1]: http://Windows_Fundamentals_for_Legacy_PCs