---
title: Installing Windows XP on DigitalOcean!
author: Seshan Ravikumar
type: post
date: 2019-05-28T18:46:53+00:00
url: /2019/05/28/installing-windows-xp-on-digitalocean/
classic-editor-remember:
  - block-editor
dsq_thread_id:
  - 7457864582
categories:
  - Uncategorized

---
<figure class="wp-block-image"><img loading="lazy" width="1024" height="640" src="https://seshan.xyz/wp-content/uploads/2019/05/Screen-Shot-2019-05-28-at-2.31.01-PM-1024x640.png" alt="" class="wp-image-80" srcset="https://seshan.xyz/wp-content/uploads/2019/05/Screen-Shot-2019-05-28-at-2.31.01-PM-1024x640.png 1024w, https://seshan.xyz/wp-content/uploads/2019/05/Screen-Shot-2019-05-28-at-2.31.01-PM-300x188.png 300w, https://seshan.xyz/wp-content/uploads/2019/05/Screen-Shot-2019-05-28-at-2.31.01-PM-768x480.png 768w, https://seshan.xyz/wp-content/uploads/2019/05/Screen-Shot-2019-05-28-at-2.31.01-PM.png 1440w" sizes="(max-width: 1024px) 100vw, 1024px" /><figcaption>_And it&#8217;s not in a VM! It&#8217;s directly running on the droplet!_</figcaption></figure> 

Whilst this shouldn&#8217;t surprise you by now, I like installing things on other things, especially when it really shouldn&#8217;t.

There is a great tutorial by GlitchWitch (<https://glitchwitch.io/blog/2018-09/windows-10-on-digitalocean/>) on how to run Windows 10 on DigitalOcean (something that may be useful to you). As it turns out though, the basic steps work for Windows XP too!

Basically you first install Windows on a QEMU/KVM VM (using VirtIO for the network and the hard drive). One change for Windows XP is you need to use the driver _floppy_ from the Fedora VirtIO Drivers to install the hard drive driver during install (F6 to install a driver!). 

Once Windows is installed, the next step is to install the VirtIO Ethernet driver from the driver ISO. Since the mouse won&#8217;t work from the DigitalOcean console, you&#8217;ll also want to set the static IP (you can find the subnet, gateway, etc from the Droplets access page). Don&#8217;t forget to enable RDP!

After that the steps basically involve booting the Droplet into recovery, transferring the raw disk image to it, and then using dd to flash it onto /dev/vda (the Droplets virtual disk). You can refer the GlitchWitch&#8217;s tutorial for the specifics.

**And that&#8217;s it!** If all goes well you&#8217;ll see Windows XP boot up in the droplet, and you can RDP into it.