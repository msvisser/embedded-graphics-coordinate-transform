//! Embedded-graphics-coordinate-transform is a helper library for rotating, mirroring and
//! transposing embedded-graphics displays.
//!
//! This library provides a single generic implementation of a coordinate transformation wrapper
//! [`CoordinateTransform`]. However, in most cases you will want to specifically rotate, mirror or
//! transpose the display, so a number of helper types are provided.
//!
//! - [`MirrorX`]: Mirror pixels along the x-axis.
//! - [`MirrorY`]: Mirror pixels along the y-axis.
//! - [`MirrorXY`]: Mirror pixels along both axes.
//! - [`TransposeXY`]: Transpose the x and y axes.
//! - [`Rotate0`]: Rotate the display by 0 degrees.
//! - [`Rotate90`]: Rotate the display by 90 degrees.
//! - [`Rotate180`]: Rotate the display by 180 degrees.
//! - [`Rotate270`]: Rotate the display by 270 degrees.
//!
//! ![A grid of displays with different coordinate transforms applied.](https://raw.githubusercontent.com/msvisser/embedded-graphics-coordinate-transform/main/assets/overview.png)
#![no_std]

#[cfg(test)]
mod tests;
mod transform;

pub use crate::transform::CoordinateTransform;

/// A coordinate transform that mirrors pixels along the x-axis of the display.
pub type MirrorX<T> = CoordinateTransform<T, true, false, false>;

/// A coordinate transform that mirrors pixels along the y-axis of the display.
pub type MirrorY<T> = CoordinateTransform<T, false, true, false>;

/// A coordinate transform that mirrors pixels along both the x and y axes of the display.
pub type MirrorXY<T> = CoordinateTransform<T, true, true, false>;

/// A coordinate transform that transposes the x and y axes of the display.
pub type TransposeXY<T> = CoordinateTransform<T, false, false, true>;

/// A coordinate transform that rotates the display by 0 degrees.
pub type Rotate0<T> = CoordinateTransform<T, false, false, false>;

/// A coordinate transform that rotates the display by 90 degrees.
pub type Rotate90<T> = CoordinateTransform<T, true, false, true>;

/// A coordinate transform that rotates the display by 180 degrees.
pub type Rotate180<T> = CoordinateTransform<T, true, true, false>;

/// A coordinate transform that rotates the display by 270 degrees.
pub type Rotate270<T> = CoordinateTransform<T, false, true, true>;
