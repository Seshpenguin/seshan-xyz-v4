---
title: Can’t Eject Your CD During a Windows XP Install?
author: Seshan Ravikumar
type: post
date: 2019-07-20T16:27:12+00:00
url: /2019/07/20/cant-eject-your-cd-during-a-windows-xp-install/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="512" height="288" src="https://seshan.xyz/wp-content/uploads/2019/07/Windows_Media_Center_on_Windows_XP.png" alt="" class="wp-image-157" srcset="https://seshan.xyz/wp-content/uploads/2019/07/Windows_Media_Center_on_Windows_XP.png 512w, https://seshan.xyz/wp-content/uploads/2019/07/Windows_Media_Center_on_Windows_XP-300x169.png 300w" sizes="(max-width: 512px) 100vw, 512px" /></figure>
</div>



So I was installing Windows XP Media Centre Edition on a Mac Mini 2009, and since the Windows XP MCE installer comes on two CDs, the plan was to just burn the first ISO onto a rewritable DVD, then when the installer wants the second one, eject the DVD, burn the second ISO, and pop it back into the Mac.

But then that didn&#8217;t work. 🙁

When the installer prompted for the second disk&#8230; I had no way to eject. I tried poking around the little file explorer window to no avail. Then I also tried a bunch of shortcuts, attempts to open stuff like the command prompt.

<blockquote class="wp-block-quote">
  <p>
    As an aside, the Mac Mini is slot loading, and there is no actual eject button on the machine itself.
  </p>
</blockquote>

**Finally, the solution was to use a USB, formatted** **as FAT32! Just copy the contents of the second ISO and Windows will pick it up&#8230;**

With a little help though. The USB won&#8217;t show up in the file picker, but you should be able to use the Drive Letter from when you formatted it (for example, on my laptop the drive showed up as E:\, so I just typed that into the box prompt). You could probably guess the drive letter too.

So yea, that&#8217;s about it. I might make a post when I get XP MCE all setup with a TV Tuner, surround sound, and everything else that&#8217;ll make it an awesome XP Home Theater PC!

<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="1024" height="847" src="https://seshan.xyz/wp-content/uploads/2019/07/Mac-mini-1st-gen-1024x847.jpg" alt="" class="wp-image-158" srcset="https://seshan.xyz/wp-content/uploads/2019/07/Mac-mini-1st-gen-1024x847.jpg 1024w, https://seshan.xyz/wp-content/uploads/2019/07/Mac-mini-1st-gen-300x248.jpg 300w, https://seshan.xyz/wp-content/uploads/2019/07/Mac-mini-1st-gen-768x635.jpg 768w" sizes="(max-width: 1024px) 100vw, 1024px" /></figure>
</div>