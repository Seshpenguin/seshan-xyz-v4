---
title: Dang it AWS. Is your instance IP Changing? Get an Elastic IP!
author: Seshan Ravikumar
type: post
date: 2018-12-06T19:19:18+00:00
url: /2018/12/06/dang-it-aws-is-your-instance-ip-changing-get-an-elastic-ip/
categories:
  - Uncategorized

---
Crisis averted, this website is still online!  
  
Let me set the scene here. It&#8217;s the middle of the Computer Club and my friend notifies me that my website is offline&#8230;  
Uh oh. I proceed to try and connect, and yep, it&#8217;s dead. I tried the direct IP and that didn&#8217;t work either. Well, it&#8217;s been a nice run of the website&#8230; Windows Server 2003 was fun. I checked the AWS panel and everything. I was pretty quick to accept the death of the site in it&#8217;s current form&#8230;

But wait! It wasn&#8217;t dead. When I got home I hopped on the AWS panel to see what was going on once more. Guess what. The public IP changed! What??? A quick Google search revealed that an AWS Instance&#8217;s Public IP will change after a restart&#8230; but why??? As it turns out you actually need to first get an Elastic IP, and associate it with the Instance. That way the instance will keep the same IP.

I&#8217;m glad I figured this out&#8230; but man this should&#8217;ve been more obvious that I need an Elastic IP to keep the instance&#8217;s public IP from changing. Hopefully this will help at least one of you 😛