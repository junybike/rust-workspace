pub trait Draw
{
    fn draw(&self);
}

pub struct Screen   
{
    // components field hold a vector of trait objects that implements the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}
// Could have used generic type parameter with trait bounds.
// Generic type parameter can only be substituted with one concrete type at a time.
// Trait object allows multiple concrete types to fill in for trait object at runtime
impl Screen
{
    pub fn run(&self)
    {
        for component in self.components.iter()
        {
            component.draw();
        }
    }
}

pub struct Button
{
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button
{
    fn draw(&self)
    {
        // drawing a button
    }
}

struct SelectBox
{
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox
{
    fn draw(&self)
    {
        // drawing select box
    }
}
