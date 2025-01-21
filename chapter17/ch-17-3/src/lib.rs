pub struct Post
{
    state: Option<Box<dyn State>>,
    content: String,
}

pub struct DraftPost
{
    content: String
}

impl Post
{
    // When creating a Post, we set state field to Some value that holds a Box
    // The Box points to a new instance of Draft struct
    // Ensures that it start out as a draft. Sets the content field to a new empty String
    pub fn new() -> Post {
        state: Some(Box::new(Draft {})),
        content:String::new(),
    }

    // Changes the Post instance 
    pub fn add_text(&mut self, text: &str)
    {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str
    {
        ""
    }

    pub fn request_review(&mut self)
    {
        if let Some(s) = self.state.take()
        {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self)
    {
        if let Some(s) = self.state.take()
        {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str
    {
        self.state.as_ref().unwrap().content(self)
    }

    // pub fn new() -> DraftPost
    // {
    //     DraftPost 
    //     {
    //         content: String::new(),
    //     }
    // }
    // pub fn content(&self) -> &str
    // {
    //     &self.content
    // }
}

impl DraftPost
{
    pub fn add_text(&mut self, text: &str)
    {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost
    {
        PendingReviewPost 
        {
            content: self.content,
        }
    }
}
pub struct PendingReviewPost
{
    content: String,
}
impl PendingReviewPost
{
    pub fn approve(self) -> Post
    {
        Post 
        {
            content: self.content,
        }
    }
}

trait State 
{
    // method is only valid when called on a Box holding the type
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str
    {
        ""
    }
}

struct Draft {}

impl State for Draft  // State trait defines behavior shared by different post states
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        self
    }
}

struct PendingReview {}

impl State for PendingReview 
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published 
{
    fn request_review(self: Box<Self>) -> Box<dyn State>
    {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State>
    {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str
    {
        &post.content
    }
}