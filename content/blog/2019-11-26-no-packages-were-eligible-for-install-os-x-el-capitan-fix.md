---
title: “No packages were eligible for install” – OS X El Capitan Fix
author: Seshan Ravikumar
type: post
date: 2019-11-26T18:57:02+00:00
url: /2019/11/26/no-packages-were-eligible-for-install-os-x-el-capitan-fix/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<figure class="wp-block-image"><img loading="lazy" width="1024" height="768" src="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.42.33-PM-1024x768.png" alt="" class="wp-image-259" srcset="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.42.33-PM-1024x768.png 1024w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.42.33-PM-300x225.png 300w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.42.33-PM-768x576.png 768w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.42.33-PM.png 2048w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure> 

If you&#8217;ve tried to install Mac OS X El Capitan (10.11), and potentially other versions of OS X&#8230; you&#8217;ll run into a problem.

<blockquote class="wp-block-quote">
  <p>
    <strong>OS X could not be installed on your computer.</strong> No packages were eligible for install. Contact the software manufacturer for assistance. Quit the installer to restart your computer and try again,
  </p>
</blockquote>

As it turns out, the certificates used to sign the OS X installer images stopped being valid sometime in 2019. And since these are older OS installers, Apple never renewed the certificates.<figure class="wp-block-image">

<img loading="lazy" width="1024" height="768" src="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.49.33-PM-1024x768.png" alt="" class="wp-image-260" srcset="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.49.33-PM-1024x768.png 1024w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.49.33-PM-300x225.png 300w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.49.33-PM-768x576.png 768w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.49.33-PM.png 2048w" sizes="(max-width: 1024px) 100vw, 1024px" /> </figure> 

**Thankfully, there is an easy solution.** All you need to do is set go to &#8220;Utilities&#8221; in the installer, select the Terminal, set the date to something before the certificates expired, and then reboot and try again. **This is the command to set the date (in this case, to January 1st, 2017):**

<pre class="wp-block-code"><code>date 0101010117</code></pre>

It&#8217;s also worth mentioning, if you are using VMWare, be sure to disable time synchronization under Advanced in the settings, and also disconnect the network.<figure class="wp-block-image">

<img loading="lazy" width="1024" height="852" src="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.50.00-PM-1024x852.png" alt="" class="wp-image-261" srcset="https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.50.00-PM-1024x852.png 1024w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.50.00-PM-300x250.png 300w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.50.00-PM-768x639.png 768w, https://seshan.xyz/wp-content/uploads/2019/11/Screen-Shot-2019-11-26-at-1.50.00-PM.png 1504w" sizes="(max-width: 1024px) 100vw, 1024px" /> </figure>