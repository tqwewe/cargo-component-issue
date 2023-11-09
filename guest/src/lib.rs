wit_bindgen::generate!({
    inline: r#"
        package component:guest;

        world example {
            export example: interface {
                hello-world: func() -> string;
            }
        }
    "#,
    exports: {
        "example": Component
    }
});

use exports::example::Guest;

struct Component;

impl Guest for Component {
    /// Say hello!
    fn hello_world() -> String {
        "Hello, World!".to_string()
    }
}
