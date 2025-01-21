use gui::{Button, Screen};

fn main()
{
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("YEs"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };
    sceen.run();

    let screen = Screen {
        components: vec![Box::new(String::from("HI"))],
    };
    screen.run();   // Error since String doesnt implement the Draw trait
}