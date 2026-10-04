async function loadBlogPosts() {
	try {
	  const response = await fetch('https://blog.seshan.xyz/wp-json/wp/v2/posts?per_page=5&_fields=title,link,date,excerpt');
	  const posts = await response.json();
	  
	  const postsContainer = document.getElementById('blog-posts');
	  
	  if (posts.length === 0) {
		postsContainer.innerHTML = '<blockquote>No posts found.</blockquote>';
		return;
	  }
	  
	  const postsHTML = posts.map(post => {
		const title = post.title.rendered;
		const date = new Date(post.date).toLocaleDateString();
		const excerpt = post.excerpt.rendered;
		
		return `
		  <blockquote class="blog-post">
			<h4><a href="${post.link}">${title}</a></h4>
			<div class="blog-post-date"><i>${date}</i></div>
			<div class="blog-post-excerpt">${excerpt}</div>
		  </blockquote>
		`;
	  }).join('');
	  
	  postsContainer.innerHTML = postsHTML;

	  document.getElementById('blog-post-count').innerText = `${posts.length} of ${response.headers.get('X-WP-Total')} posts`;
	  
	} catch (error) {
	  document.getElementById('blog-posts').innerHTML = `<blockquote>Unable to load blog posts:<br /> ${error.message}</blockquote>`;
	  console.error('Error loading blog posts:', error);
	}
}

loadBlogPosts();