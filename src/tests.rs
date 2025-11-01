use crate::{MirrorX, MirrorY, Rotate0, Rotate180, Rotate270, Rotate90, TransposeXY};
use embedded_graphics::mock_display::MockDisplay;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics_core::primitives::Rectangle;

#[test]
fn mirror_x() {
    let mut display = MirrorX::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::new(60, 0), Size::new_equal(4))
    );
}

#[test]
fn mirror_y() {
    let mut display = MirrorY::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::new(0, 60), Size::new_equal(4))
    );
}

#[test]
fn transpose_xy() {
    let mut display = TransposeXY::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new(4, 2))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::zero(), Size::new(2, 4))
    );
}

#[test]
fn rotate0() {
    let mut display = Rotate0::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::zero(), Size::new_equal(4))
    );
}

#[test]
fn rotate90() {
    let mut display = Rotate90::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::new(0, 60), Size::new_equal(4))
    );
}

#[test]
fn rotate180() {
    let mut display = Rotate180::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::new(60, 60), Size::new_equal(4))
    );
}

#[test]
fn rotate270() {
    let mut display = Rotate270::new(MockDisplay::<BinaryColor>::new());

    Rectangle::new(Point::zero(), Size::new_equal(4))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    assert_eq!(
        display.affected_area(),
        Rectangle::new(Point::new(60, 0), Size::new_equal(4))
    );
}
