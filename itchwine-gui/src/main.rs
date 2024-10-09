use gtk::{prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, ButtonExt}, Application, Builder, Button, Window};


const APPLICATION_ID: &str = "com.github.itchwine.itchwine-gui";


fn main() {
    let app = Application::builder()
    .application_id(APPLICATION_ID)
    .build();

    app.connect_activate(main_ui);

    app.run();
}

fn main_ui(app: &Application) {
    let builder = Builder::from_string(include_str!("resources/ui/main.ui"));

    let window: Window = builder.object("main_window").expect("Couldn't find the window");
    let button: Button = builder.object("button").expect("Couldn't find the button");

    button.connect_clicked(|_| {
        println!("Button clicked!");
    });

    window.present();
}
