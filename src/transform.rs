use core::ops::{Deref, DerefMut};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    primitives::Rectangle,
    Pixel,
};

/// A generic coordinate transform that can both mirror along the x and y axes and transpose.
///
/// This is a generic implementation of the coordinate transforms for embedded-graphics draw
/// targets. These coordinate transforms can mirror along the x-axis, mirror along the y-axis, and
/// transpose the x and y axes. These operations are always applied in this specific order when
/// enabled.
///
/// This coordinate transform can wrap any implementation of [`DrawTarget`]. Therefore, it solely
/// relies on remapping pixel coordinates as they are supplied to draw calls and passing them on
/// to the wrapped [`DrawTarget`]. This means that it cannot use any optimized screen rotation
/// capabilities that might be available in your display implementation. Optimized implementations
/// of `fill_solid` and `clear` are provided, however `fill_contiguous` will fall back on the
/// `draw_iter` implementation.
pub struct CoordinateTransform<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool>
{
    draw_target: D,
}

impl<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool>
    CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
{
    /// Create a new `CoordinateTransform` which wraps a supplied [`DrawTarget`].
    pub fn new(draw_target: D) -> CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE> {
        CoordinateTransform { draw_target }
    }

    /// Unwraps this `CoordinateTransform`, returning the underlying [`DrawTarget`].
    pub fn into_inner(self) -> D {
        self.draw_target
    }
}

impl<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> OriginDimensions
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
where
    D: OriginDimensions,
{
    fn size(&self) -> Size {
        let size = self.draw_target.size();
        if TRANSPOSE {
            Size::new(size.height, size.width)
        } else {
            size
        }
    }
}

impl<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> DrawTarget
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
where
    D: DrawTarget + OriginDimensions,
{
    type Color = D::Color;
    type Error = D::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width = self.size().width as i32;
        let height = self.size().height as i32;

        self.draw_target
            .draw_iter(pixels.into_iter().map(|Pixel(mut point, color)| {
                if MIRROR_X {
                    point.x = width - 1 - point.x;
                }

                if MIRROR_Y {
                    point.y = height - 1 - point.y;
                }

                if TRANSPOSE {
                    point = Point::new(point.y, point.x);
                }

                Pixel(point, color)
            }))
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let mut area = *area;

        if MIRROR_X {
            area.top_left.x = self.size().width as i32 - area.top_left.x - area.size.width as i32;
        }

        if MIRROR_Y {
            area.top_left.y = self.size().height as i32 - area.top_left.y - area.size.height as i32;
        }

        if TRANSPOSE {
            area = Rectangle::new(
                Point::new(area.top_left.y, area.top_left.x),
                Size::new(area.size.height, area.size.width),
            );
        }

        self.draw_target.fill_solid(&area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.draw_target.clear(color)
    }
}

impl<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> Deref
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
{
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.draw_target
    }
}

impl<D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> DerefMut
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.draw_target
    }
}

impl<T, D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> AsRef<T>
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
where
    T: ?Sized,
    <CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE> as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl<T, D, const MIRROR_X: bool, const MIRROR_Y: bool, const TRANSPOSE: bool> AsMut<T>
    for CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE>
where
    T: ?Sized,
    <CoordinateTransform<D, MIRROR_X, MIRROR_Y, TRANSPOSE> as Deref>::Target: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut().as_mut()
    }
}
