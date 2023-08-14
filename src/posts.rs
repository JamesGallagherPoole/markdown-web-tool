pub struct Post {
    pub title: String,
    pub date: String,
    pub content: String,
    pub path: String,
}

pub fn create_recent_posts_html(posts: &Vec<Post>, num_posts: usize) -> String {
    let mut recent_posts_html =
        String::from("<div id=\"recent-posts\">\n<h2>Recent Posts</h2>\n<ul>");
    for post in posts.iter().rev().take(num_posts) {
        let post_html = format!("<li><a href=\"{}\">{}</a></li>\n", post.path, post.title);
        recent_posts_html.push_str(&post_html);
    }
    recent_posts_html.push_str("</ul>\n</div>\n");
    recent_posts_html
}
