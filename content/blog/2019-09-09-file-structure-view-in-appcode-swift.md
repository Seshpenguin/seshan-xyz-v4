---
title: File Structure View in AppCode (Swift)
author: Seshan Ravikumar
type: post
date: 2019-09-09T22:48:53+00:00
url: /2019/09/09/file-structure-view-in-appcode-swift/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
Recently I have been dabbling in Swift, but Apple&#8217;s XCode IDE has left a lot to be desired. Thankfully, JetBrains has two IDEs that work with Swift, CLion (great for basic Swift CLI/Server applications), and AppCode (a more direct competitor to XCode, with the ability to open XCode projects and debug on iOS).

Like Rider, which is JetBrain&#8217;s Visual Studio competitor, AppCode is designed to mimic the default behaviour of it&#8217;s origin IDE. In this case, I noticed that, like XCode, AppCode wouldn&#8217;t show files outside of the XCode project. In my case this meant a folder containing some HTML and CSS files.

<div class="wp-block-image">
  <figure class="aligncenter"><img loading="lazy" width="450" height="354" src="https://seshan.xyz/wp-content/uploads/2019/09/Screen-Shot-2019-09-09-at-6.41.54-PM.png" alt="" class="wp-image-214" srcset="https://seshan.xyz/wp-content/uploads/2019/09/Screen-Shot-2019-09-09-at-6.41.54-PM.png 450w, https://seshan.xyz/wp-content/uploads/2019/09/Screen-Shot-2019-09-09-at-6.41.54-PM-300x236.png 300w" sizes="(max-width: 450px) 100vw, 450px" /></figure>
</div>

**Thankfully, it&#8217;s easy to go back to the regular file structure view. Just click on &#8220;Project&#8221; above where your source files are listed and** **select &#8220;Files&#8221; in the drop down.**

Now your Swift Project in AppCode will display all files, including non-project ones!