---
title: Running Classic (Mac OS 9) Finder “Full Screen” on Mac OS X!
author: Seshan Ravikumar
type: post
date: 2020-08-15T16:00:00+00:00
url: /2020/08/15/running-classic-mac-os-9-finder-full-screen-on-mac-os-x/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
From Mac OS 10.0 to 10.4, Apple shipped &#8220;Classic&#8221;, a way to run classic Mac OS apps on OS X, by booting a Mac OS 9 environment. This works pretty well (compared to the &#8220;[Blue Box][1]&#8221; used during the development versions of OS X, which was much more virtual machine like, using a disk image and was completely full), since Mac OS 9 windows integrated with OS X windows.

<div class="wp-block-image">
  <figure class="aligncenter size-large"><img loading="lazy" width="1024" height="768" src="https://seshan.xyz/wp-content/uploads/2020/08/tiger-os9-1.png" alt="" class="wp-image-310" srcset="https://seshan.xyz/wp-content/uploads/2020/08/tiger-os9-1.png 1024w, https://seshan.xyz/wp-content/uploads/2020/08/tiger-os9-1-300x225.png 300w, https://seshan.xyz/wp-content/uploads/2020/08/tiger-os9-1-768x576.png 768w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure>
</div>

While you can&#8217;t get access to the OS 9 Finder by default in Classic (and thus the amazing About This Computer window), you can actually get it to work with a bit of modification! These instructions are adapted from this MacRumors forum post: <https://forums.macrumors.com/threads/full-screen-classic-on-tiger.2106826/>

  * Make sure you have a functioning Classic Environment
  * Download [ResEdit][2]
  * Go to the OS 9 System Folder, right click on &#8220;Finder&#8221; and hit Duplicate.
  * Rename the duplicate to &#8220;Finder (Classic)&#8221; and move it to the OS 9 Applications folder
  * Open ResEdit and open your copy of Finder
  * On the top bar, under &#8220;File&#8221;, go to &#8220;Get Info for Finder (Classic)&#8221;
  * Change Type to &#8220;APPL&#8221; and Creator to &#8220;aplt&#8221;
  * Save and exit ResEdit.
  * Done! You can launch Finder!

It&#8217;s worth mention that while this is pretty fun to play around with, it&#8217;s also really unstable and likes to stop existing randomly (even when it&#8217;s just sitting there doing nothing).

 [1]: https://en.wikipedia.org/wiki/List_of_macOS_components#Classic
 [2]: http://www.mac.org/utilities/resedit/