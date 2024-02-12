---
title: Getting WPA2 Personal/Enterprise WiFi support in OS X Tiger (10.4)
author: Seshan Ravikumar
type: post
date: 2020-08-13T14:37:24+00:00
url: /2020/08/13/getting-wpa2-personal-enterprise-wifi-support-in-os-x-tiger-10-4/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
If you&#8217;re anything like me, you enjoy using PowerPC Macs! One interesting problem I ran into was that Tiger seemingly didn&#8217;t support WPA2 encryption for WiFi (only presenting an option for WPA personal). This was weird, both because it worked fine in Leopard, and because the latest 10.4.11 combo update should support WPA2. Making matters even more interesting, I&#8217;m pretty sure I had WPA2 working fine on Tiger a few years ago.

Anyway, I found a solution posted by &#8220;[iluvmacs99][1]&#8221; on the MacRumors PowerPC forum.

  * Open System Preferences
  * Go to Network, Select &#8220;Airport&#8221; from the &#8220;Show&#8221; dropdown menu
  * Select the Airport tab (first tab), then select &#8220;By default Join Preferred Networks&#8221;.
  * An empty list will appear, select the &#8220;+&#8221; icon on the bottom.
  * You can now select the Network Name and Security type!

<div class="wp-block-image">
  <figure class="aligncenter size-large"><img loading="lazy" width="595" height="553" src="https://seshan.xyz/wp-content/uploads/2020/08/airport-settings.png" alt="" class="wp-image-306" srcset="https://seshan.xyz/wp-content/uploads/2020/08/airport-settings.png 595w, https://seshan.xyz/wp-content/uploads/2020/08/airport-settings-300x279.png 300w" sizes="(max-width: 595px) 100vw, 595px" /></figure>
</div>



I have no idea why you have to do this (maybe newer Wireless networks mess with Tiger&#8217;s security auto selection?), but either way, hope this helps!

 [1]: https://forums.macrumors.com/members/iluvmacs99.1172278/