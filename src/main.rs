use vizia::prelude::*;
use vizia_grid::Grid;

fn main() {
    Application::new(|cx| {
        Grid::new(cx, 10, 10, |cx, x, y| {
            Button::new(
                cx,
                move |_| println!("x: {x}, y: {y}"),
                |cx| Label::new(cx, ""),
            )
            .background_color(Color::black())
            .border_color(Color::gray())
            .border_width(Pixels(1.0))
            .border_radius(Pixels(0.0))
        })
        .size(Stretch(1.0));
    })
    .title("Prueba")
    .inner_size((800, 800))
    .resizable(false)
    .run();
}
