use vizia::prelude::*;

/// A grid of the selected dimensions with the same contents in every cell.
/// The content function provides access to the cell's position within the grid.
pub struct Grid {}

impl Grid {
    /// Creates a new Grid.
    /// # Examples
    /// ```
    /// # use vizia_core::prelude::*;
    /// #
    /// # let cx = &mut Context::default();
    /// #
    /// Grid::new(cx, 3, 3, |_, x, y| println!("x: {}, y: {}", x, y), |cx| { Label::new("test") })
    /// ```
    pub fn new<C, V>(cx: &mut Context, width: usize, height: usize, content: C) -> Handle<Self>
    where
        C: Fn(&mut Context, usize, usize) -> Handle<V>,
        V: 'static + View,
    {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                for y in 0..height {
                    HStack::new(cx, |cx| {
                        for x in 0..width {
                            (content)(cx, x, y).size(Stretch(1.0));
                        }
                    })
                    .size(Stretch(1.0));
                }
            })
            .size(Stretch(1.0));
        })
    }
}

impl View for Grid {
    fn element(&self) -> Option<&'static str> {
        Some("grid")
    }
}
