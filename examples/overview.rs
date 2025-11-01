use embedded_graphics::{
    mono_font::ascii::FONT_8X13,
    mono_font::MonoTextStyle,
    primitives::{Primitive, PrimitiveStyle, Triangle},
    text::{Alignment, Text},
};
use embedded_graphics_coordinate_transform::{
    MirrorX, MirrorY, Rotate0, Rotate180, Rotate270, Rotate90,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{AnchorPoint, OriginDimensions, Point, Size},
    pixelcolor::{Rgb888, RgbColor, WebColors},
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_simulator::{
    MultiWindow, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
};
use std::convert::Infallible;

/// Render a test scene with some rectangles and a piece of test.
pub fn render_scene<D, E>(display: &mut D) -> Result<(), E>
where
    D: DrawTarget<Color = Rgb888, Error = E> + OriginDimensions,
{
    display.clear(Rgb888::WHITE)?;

    Rectangle::new(Point::zero(), Size::new(4, 4))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
        .draw(display)?;

    Rectangle::with_corners(
        Point::new(0, 8),
        Point::new(0, display.size().height as i32 - 1),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
    .draw(display)?;

    Rectangle::with_corners(
        Point::new(2, 8),
        Point::new(3, display.size().height as i32 - 1),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
    .draw(display)?;

    Rectangle::with_corners(
        Point::new(8, 3),
        Point::new(display.size().width as i32 - 1, 3),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
    .draw(display)?;

    Triangle::new(
        Point::new(
            display.size().width as i32 - 1,
            display.size().height as i32 - 1,
        ),
        Point::new(
            display.size().width as i32 - 9,
            display.size().height as i32 - 1,
        ),
        Point::new(
            display.size().width as i32 - 1,
            display.size().height as i32 - 9,
        ),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
    .draw(display)?;

    Text::with_alignment(
        "Hello,\nworld!",
        display.bounding_box().center(),
        MonoTextStyle::new(&FONT_8X13, Rgb888::BLACK),
        Alignment::Center,
    )
    .draw(display)?;

    Ok(())
}

/// Calculate the display offset based on the window and display size.
fn display_offset(window_size: Size, display_size: Size, anchor_point: AnchorPoint) -> Point {
    let layout_rect = Rectangle::new(Point::zero(), window_size).offset(-20);
    layout_rect.resized(display_size, anchor_point).top_left
}

/// Run the overview example.
fn main() -> Result<(), Infallible> {
    // Create 6 displays to show off the 4 rotation and 2 mirror transforms.
    let display_size = Size::new(128, 64);
    let mut original_display = Rotate0::new(SimulatorDisplay::<Rgb888>::new(display_size));
    let mut transformed_display_rot90 =
        Rotate90::new(SimulatorDisplay::<Rgb888>::new(display_size));
    let mut transformed_display_rot180 =
        Rotate180::new(SimulatorDisplay::<Rgb888>::new(display_size));
    let mut transformed_display_rot270 =
        Rotate270::new(SimulatorDisplay::<Rgb888>::new(display_size));
    let mut transformed_display_mirror_x =
        MirrorX::new(SimulatorDisplay::<Rgb888>::new(display_size));
    let mut transformed_display_mirror_y =
        MirrorY::new(SimulatorDisplay::<Rgb888>::new(display_size));

    // Render the scene to each display.
    render_scene(&mut original_display)?;
    render_scene(&mut transformed_display_rot90)?;
    render_scene(&mut transformed_display_rot180)?;
    render_scene(&mut transformed_display_rot270)?;
    render_scene(&mut transformed_display_mirror_x)?;
    render_scene(&mut transformed_display_mirror_y)?;

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let display_output_size = original_display.output_size(&output_settings);

    // Create a window that can hold 2x3 displays with 20 pixels of padding.
    let window_size = (display_output_size + Size::new_equal(30)).component_mul(Size::new(2, 3));
    let mut window = MultiWindow::new("Overview", window_size);
    window.clear(Rgb888::CSS_GRAY);

    // Add the displays to the window
    window.add_display(
        &original_display,
        display_offset(window_size, display_output_size, AnchorPoint::TopLeft),
        &output_settings,
    );
    window.add_display(
        &transformed_display_rot90,
        display_offset(window_size, display_output_size, AnchorPoint::TopRight),
        &output_settings,
    );
    window.add_display(
        &transformed_display_rot180,
        display_offset(window_size, display_output_size, AnchorPoint::CenterLeft),
        &output_settings,
    );
    window.add_display(
        &transformed_display_rot270,
        display_offset(window_size, display_output_size, AnchorPoint::CenterRight),
        &output_settings,
    );
    window.add_display(
        &transformed_display_mirror_x,
        display_offset(window_size, display_output_size, AnchorPoint::BottomLeft),
        &output_settings,
    );
    window.add_display(
        &transformed_display_mirror_y,
        display_offset(window_size, display_output_size, AnchorPoint::BottomRight),
        &output_settings,
    );

    'running: loop {
        // Update each display and render to the window.
        window.update_display(&original_display);
        window.update_display(&transformed_display_rot90);
        window.update_display(&transformed_display_rot180);
        window.update_display(&transformed_display_rot270);
        window.update_display(&transformed_display_mirror_x);
        window.update_display(&transformed_display_mirror_y);
        window.flush();

        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'running;
            }
        }
    }

    Ok(())
}
