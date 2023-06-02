use typechain::{chain, use_chains};
use_chains![typechain::HasTags];


chain!(Post => {
    @HasTags
    static tags: Vec<&'static str> = vec!["post"];

    const title: String;
    const comments: Vec<Comment>;
});

chain!(Comment => {
    @HasTags
    static tags: Vec<&'static str> = vec!["comment"];

    const body: String;
});

impl Comment {
    pub fn new(body: String) -> Self {
        Self {
            body
        }
    }
}

#[test]
fn test_tags() {
    let comments = vec![
        Comment::new("Goodbye, world! (1/3)".to_string()),
        Comment::new("Goodbye, world! (2/3)".to_string()),
        Comment::new("Goodbye, world! (3/3)".to_string()),
    ];

    let post = Post {
        title: "Hello, world!".to_string(),
        comments,
    };

    let has_tags: &HasTags = &post;

    assert_eq!(has_tags.tags(), vec!["post"]);
}