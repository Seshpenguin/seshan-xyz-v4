---
title: WHY WON’T MY REACT-NATIVE-ELEMENTS BUTTON STRETCH!?
author: Seshan Ravikumar
type: post
date: 2019-07-30T18:28:02+00:00
url: /2019/07/30/why-wont-my-react-native-elements-button-stretch/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
If you were like me and spent **way too long** trying to get your React Native Elements button to fill the parent width when placed inside a _flexDirection: &#8216;row&#8217;_ view&#8230; here is your solution:

<pre class="wp-block-code"><code>containerStyle={{flex: 1}}</code></pre>

Just throw that prop on your button and EVERYTHING WILL WORK! As always make sure your container also stretches by putting &#8216;flex: 1&#8217; or setting the width style as 100% (this works well if you have a ScrollView).