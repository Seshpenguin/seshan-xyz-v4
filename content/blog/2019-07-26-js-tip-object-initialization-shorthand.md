---
title: 'JS Tip: Object Initialization Shorthand!'
author: Seshan Ravikumar
type: post
date: 2019-07-26T23:59:59+00:00
url: /2019/07/26/js-tip-object-initialization-shorthand/
classic-editor-remember:
  - block-editor
categories:
  - Uncategorized

---
Let&#8217;s say your writing some React code, and you end up doing something like this:

<pre class="wp-block-code"><code>let a = 123;
let b = 456;
let c = 789;
this.setState({ a: a, b: b, c: c })</code></pre>

Having to write out variable names twice in your setState() statement seems a bit inefficient. Thankfully, there is a better way in ES2015:

<pre class="wp-block-code"><code>let a = 123;
let b = 456;
let c = 789;
this.setState({ a, b, c })</code></pre>

That&#8217;s so much nicer! The object will be automatically initialized with the name of the variable as the property name. Awesome, isn&#8217;t it?

<blockquote class="wp-block-quote">
  <p>
    By the way, I hope you guys like this new segment I&#8217;m calling JS Tips. Anytime I find little useful JS tidbits I feel like sharing, I&#8217;ll create a new installment of JS Tips!
  </p>
</blockquote>