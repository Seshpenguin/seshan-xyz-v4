---
title: 'WordPress Performance Tip: Use the System Cron!'
author: Seshan Ravikumar
type: post
date: 2019-06-08T13:00:31+00:00
url: /2019/06/08/wordpress-performance-tip-use-the-system-cron/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
One of the problems with PHP Web Apps is that they are only ever running for the duration of an HTTP Request. What I mean by that is that websites powered by WordPress, for example, aren&#8217;t continuously running on a host server. The only time WordPress gets to do any work is when a user requests a page, and the webserver invokes PHP.

This is great, but sometimes WordPress needs to do other background tasks, such as internal tasks like random housekeeping, or tasks you scheduled like a scheduled post. The way WordPress handles this is through WP-Cron.

Basically, WP-Cron is a file (wp-cron.php) that is run by WordPress every time you load a page. When it&#8217;s invoked, it checks a list of scheduled tasks and sees if any need to be run. This works pretty well, but is also **quite the performance issue.** Since this happens when someone loads a page, it&#8217;s an extra thing WordPress is busy doing besides actually serving the page. Further, more time sensitive tasks, may not actually happen on time. For example, if a post is set to be released at 1:00PM, it may not actually make it out at exactly 1:00PM. It will be posted sometime after 1:00PM, whenever a request is made to the server.

**So what is the solution? Use the system cron!**

Unix-like systems such as GNU/Linux, macOS, BSD, etc, have a system called &#8220;cron&#8221; (which WP-Cron was named after). What we can do is use cron to periodically call wp-cron.php. This way, WordPress can do background tasks at regular intervals, and not clog up your users requests!

<hr class="wp-block-separator" />

The first thing to do is disable running WP-Cron on every single request. Head over to your wp-config.php file and add the following somewhere near the bottom:

<pre class="wp-block-code"><code>define('DISABLE_WP_CRON', true);</code></pre>

Once that&#8217;s done, it&#8217;s time to add a cronjob! A cronjob is basically a single line that describe when a command should be run. The [WordPress developer page][1] has a nice infographic of what a cronjob looks like:

<div class="wp-block-image">
  <figure class="aligncenter is-resized"><img loading="lazy" src="https://developer.wordpress.org/files/2014/10/plugin-wp-cron-cron-scheduling-300x150.png" alt="plugin-wp-cron-cron-scheduling" class="wp-image-11813" width="396" height="198" /></figure>
</div>

For my needs, I want wp-cron.php to run every 15 minutes. So, add the following line to the bottom of the &#8220;`nano /etc/crontab`&#8221; file (atleast on Ubuntu) on your server (changing YOUR\_SITE\_URL to something like localhost, or your sites domain name). This technically doesn&#8217;t have to be on the same server as your WordPress site!

<pre class="wp-block-code"><code>*/15 * * * * root wget --delete-after http://YOUR_SITE_URL/wp-cron.php</code></pre>

What this line does is use &#8220;wget&#8221; to call the wp-cron.php file on your server every 15 minutes. The &#8220;&#8211;delete-after&#8221; is to make sure wget doesn&#8217;t spam your host will random files.

All you need to do now is restart the Cron daemon to load the new changes:

<pre class="wp-block-code"><code># sudo systemctl restart cron</code></pre>

**And that&#8217;s it! Now your WordPress site is setup to run background tasks at regular intervals and more efficiently! Enjoy!**

Btw, if you&#8217;ve noticed that the there have been consecutive blog posts that have been posted at specific times: you can thank Cron!

 [1]: https://developer.wordpress.org/plugins/cron/hooking-wp-cron-into-the-system-task-scheduler/