---
title: Hello World!
author: Seshan Ravikumar
type: post
date: 2018-11-12T19:07:30+00:00
url: /2018/11/12/hello-world-2/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
<blockquote class="wp-block-quote">
  <p>
    May 2019 Update: This blog post was the first post on my old, custom blog system. Now that I got bored of it, we have a new blog systems.
  </p>
  
  <p>
    Just keep in mind that this post no longer reflects how the blog works nowadays.
  </p>
</blockquote>

Hey! I see you have stumbled upon my blog&#8230; I just finished making it as I write this post, let me tell you how it works!  
So, in true Seshan fashion, and since I made the great decision to run this website on Windows Server 2003, I decided to write the blog &#8220;software&#8221; in ASP. And yes, that&#8217;s Classic ASP with VBScript, none of this fancy so called ASP.NET around here (actually it&#8217;s because ASP.NET wasn&#8217;t cooperating)! 

The blog is actually really simple, basically I have a blog folder which contains a bunch of files numbered starting from 0 (that is the post ID) containing the post content. Then, there is a subfolder called meta with files of identical names to the blog folder that contain the post title.  
The Default.asp file (the one you are reading this post from) is where things get interesting. The static part is just some HTML and CSS (Materialize). The ASP/VBScript part does the following (simplified).



  * 1. Loop through each file in the blog folder, store it&#8217;s contents in an array
      * The array size starts at zero, but is expanded using &#8220;REDIM&#8221; to a size of &#8220;postCount&#8221; (a variable that increases with each loop, i.e each post.)
  * 2. Loop from postCount to zero (since postCount is the id of the latest post)
      * Write the contents of the post aswell as the post title to the page (in a Materialize collapsible)
  * 3. Done! It&#8217;s not very complicated&#8230;

So yea, it&#8217;s nothing too impressive. I might expand it with more features later. Want RSS? Too bad, though if i&#8217;m bored then maybe&#8230;  
That&#8217;s it for now!