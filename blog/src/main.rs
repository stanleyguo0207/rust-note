use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("This is my first blog.");
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("This is my first blog.", post.content());
}
