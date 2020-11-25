use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{Event, Style};

fn main() {
    let mut window =
        RenderWindow::new((800, 600), "Retroland", Style::DEFAULT, &Default::default());

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        window.clear(Color::BLUE);
        window.display();
    }
}
