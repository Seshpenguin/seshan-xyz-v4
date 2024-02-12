---
title: Windows Server on KVM in Docker on DigitalOcean
author: Seshan Ravikumar
type: post
date: 2018-12-12T21:23:17+00:00
url: /2018/12/12/windows-server-on-kvm-in-docker-on-digitalocean/
categories:
  - Uncategorized

---
Yep, you read the title right. I&#8230; uh&#8230; yea I installed Windows Server 2008 on KVM in a Docker container on a DigitalOcean Droplet. For a production environment&#8230;  
  
So as some of you may know I work for ConnectUS, a company who is designing a website/service for the educational sector. It&#8217;s not released yet, so I don&#8217;t really know how much I can actually say. Anyway, we needed a mail server for the usual verify your account, and also for some pretty integral parts of the website. Now, we already use GNU/Linux for everything else, and all we needed was basically an SMTP server, so something like Postfix would&#8217;ve been the reasonable thing to do. Of course, me being myself wouldn&#8217;t satisfied that that. Nope, we need to make this interesting!  
  
So the plan was to install Microsoft Exchange Server (which didn&#8217;t end up happening sadly, setting up Active Directory and everything else required for enterprise Microsoft software was not really worth it, considering the timeline too). Oh and btw, if you ever wanted to see some crazy stuff, look up the recommend system specs for Exchange 2019, it&#8217;s like 128GB of RAM or something stupid like that. Anyway, I digress. I choose Windows Server 2008 becuase of the maximum fun factor, plus it&#8217;s still in support. It also uses way less resources than Server 2019 would.  
  
So began the adventure. I&#8217;ve already installed Windows a number of times in QEMU+KVM, Virt IO en all. Now, we already use Docker extensively, to host the various components of the website ( backend, frontend, db, proxies, etc), so why not put the mail server VM in a container too? As it turns out, I wasn&#8217;t the only one with this idea, and I found a  [pretty cool GitHub project][1] that was a Docker container (CentOS) with QEMU. Seems pretty simple, right? Wrong! It&#8217;s actually even cooler. The VMs network card is actually mapped to the Containers IP, meaning you can manage all you networking (like exposing ports) right with Docker, and treat everything running inside the VM just as if it was a normal container. It has some other nice things too, like a serial port on the guest thats mapped to the docker container&#8217;s logs.  
  
First thing to do was to install Windows onto a qcow2 image (using the Windows VirtIO drivers for storage). After that I forked the repo to make some changes to the QEMU options (including making it more Windows friendly). Finally spin up the container and boom! VM on Docker:  
<figure class="wp-block-image">

![][2] </figure> 

Awesome! Now, it wasn&#8217;t all that smooth. My first mistake was forgetting to enable RDP (the container doesn&#8217;t expose any way of access the QEMU QUI, and the built-in VNC doesn&#8217;t work since the containers IP is mapped to inside the VM, not the QEMU process). Ok, no problem just shut off the container, install RDP, then try again. Then came issue #2, I realized that I never installed the VirtIO network drivers. Thats fine, just shut off the container and replace the network card for now. The the really stupid part: I forgot the RDP port. I spent so long wondering why I couldn&#8217;t connect. Turns out I forwarded port 3386 instead of 3389. \*sigh\*  
  
But yea, that&#8217;s basically it&#8230; yay? For those wondering, instead of Exchange Server I went with hMailServer. I actually totally forgot about it&#8217;s existence. I actually tried using it a really long time ago as a kid (idk make 11 or 12), so there was some nostalgia there (and hey, this time I got the software to work!).

 [1]: https://github.com/BBVA/kvm
 [2]: http://legacy.seshan.xyz/flow/files/Screenshot at 2018-12-12 17-21-32.png