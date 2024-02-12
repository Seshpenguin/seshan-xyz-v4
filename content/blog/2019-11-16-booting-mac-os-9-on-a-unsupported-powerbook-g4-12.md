---
title: Booting Mac OS 9 on a (unsupported) PowerBook G4 12″!
author: Seshan Ravikumar
type: post
date: 2019-11-16T15:42:03+00:00
url: /2019/11/16/booting-mac-os-9-on-a-unsupported-powerbook-g4-12/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
After playing with some old Macs in my Computer Engineering class, I had a itch to play with some classic Mac OS on some of my own Macs. Sadly, the only PowerPC Mac I have is a 12&#8243; PowerBook G4. It&#8217;s an awesome machine, but it&#8217;s too new to support OS 9.

Or atleast I thought&#8230;

Thanks to various community efforts (OS 9 still has quite the following even after Apple introduced OS X), OS 9 can boot on all sorts of newer PowerPC Apple hardware!

Before I begin, here is a list of a few of the sources that were pretty helpful:

  * http://macos9lives.com/smforum/index.php/topic,3923.0.html?PHPSESSID=ar23ei5ui7u3ea74uk8g0kral6
  * https://discussions.apple.com/thread/971151
  * https://www.thinkclassic.org/viewtopic.php?pid=4889#p4889
  * http://macos9lives.com/smforum/index.php?topic=2143.0
  * And a few more&#8230;

OK! So the first thing you&#8217;ll want to do is to grab yourself a copy of [OS 9.2.2 for Unsupported G4s][1]. Burn it to a CD (or DVD, if your Mac has a DVD drive).

<blockquote class="wp-block-quote">
  <p>
    I wouldn&#8217;t even think about USB booting&#8230; It might be possible, but I don&#8217;t want to try and deal with that&#8230; yet.
  </p>
</blockquote>

Next! Boot into OpenFirmware. We need to modify a few variables here to trick OS 9 into thinking we have a different CPU. **Just type eac**h **line, and press enter after.**

<pre class="wp-block-code"><code>nvedit
dev /cpus/@0
80010201 encode-int " cpu-version" property</code></pre>

**Now press Ctrl-C to exit nvedit**. Then finish off with these commands to save these custom settings:

<pre class="wp-block-code"><code>nvstore 
setenv use-nvramrc? true
reset-all</code></pre>

After entering the last command, the computer will restart. Just hold down the &#8220;C&#8221; key to boot from the CD/DVD drive! 

<hr class="wp-block-separator" />
<figure class="wp-block-image">
<img loading="lazy" width="576" height="1024" src="https://seshan.xyz/wp-content/uploads/2019/11/EI8l_fPXYAEcKXe-576x1024.jpg" alt="" class="wp-image-253" srcset="https://seshan.xyz/wp-content/uploads/2019/11/EI8l_fPXYAEcKXe-576x1024.jpg 576w, https://seshan.xyz/wp-content/uploads/2019/11/EI8l_fPXYAEcKXe-169x300.jpg 169w, https://seshan.xyz/wp-content/uploads/2019/11/EI8l_fPXYAEcKXe.jpg 675w" sizes="(max-width: 576px) 100vw, 576px" /> </figure> 

_And there you go!_ You can now enjoy all the classic Mac OS programs, bask in the glory of the **Platinum sounds**, and whatever else you want to do. Do keep in mind though, not everything works properly. Notably absent is WiFi (ethernet does work), and Graphics Accelerations (so games are a no go, **but OS 9 does still run at the full screen resolution** and is still pretty snappy).

 [1]: http://macos9lives.com/smforum/index.php?topic=2143.0