// use std::ops::{Bound, RangeBounds, RangeInclusive};
//
// use crate::Style;
// use btree_range_map::{
//     generic::map::{IntoIter as MapIntoIter, Iter as MapIter},
//     DefaultMapContainer as MapSlab, RangeMap,
// };
//
// /// [`Mask`] saves [`styles`] for specified ranges. It is low-level structure for creation of
// /// styled [`text`]. Look at high-level functions if you just want create styled text: TODO
// ///
// /// [`styles`]: Style
// /// [`text`]: crate::Text
// #[derive(Debug, Clone, Default)]
pub struct Mask {
    // map: RangeMap<usize, Style>,
}
//
// pub struct AnyRange {
//     pub start: Bound<usize>,
//     pub end: Bound<usize>,
// }
//
// /// Item of [`Mask`]: range and [`style`] for this range.
// ///
// /// [`style`]: Style
// pub type RangeStyle = (AnyRange, Style);
//
// /// An iterator over the items of [`Mask`].
// ///
// /// The iterator element type is `(RangeInclusive<usize>, &'a Style)`.
// ///
// /// This struct is created by the [`iter`] method on [`Mask`]. See its documentation
// /// for more.
// ///
// /// [`iter`]: Mask::iter
// #[must_use = "Iterators are lazy and do nothing unless consumed"]
// pub struct Iter<'a> {
//     inner: MapIter<'a, usize, Style, MapSlab<usize, Style>>,
// }
//
// /// An owning iterator over the items of [`Mask`].
// ///
// /// The iterator element type is `(RangeInclusive<usize>, Style)`.
// ///
// /// This struct is created by the [`into_iter`] method on [`Mask`].
// ///
// /// [`into_iter`]: IntoIterator::into_iter
// #[must_use = "Iterators are lazy and do nothing unless consumed"]
// pub struct IntoIter {
//     inner: MapIntoIter<usize, Style, MapSlab<usize, Style>>,
// }
//
// impl Mask {
//     /// Create empty `Mask`.
//     pub fn new() -> Self {
//         Self::default()
//     }
//
//     /// Add `style` for specified `range`. It merges all styles for overlapping ranges.
//     ///
//     /// # Panics
//     ///
//     /// Panics if `start_bound` of range > `end_bound`
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use yatui_text::{Mask, Style, Color};
//     ///
//     /// let mut mask = Mask::new();
//     /// mask.add(0..=1, Style::new().fg(Color::Yellow));
//     /// mask.add(0..2, Style::new().bg(Color::Green));
//     /// ```
//     pub fn add(&mut self, range: impl RangeBounds<usize>, style: Style) {
//         self.map.update(utils::bound_to_inclusive(range), |styles| {
//             Some(match styles {
//                 Some(cur_style) => cur_style.merge(style),
//                 None => style,
//             })
//         })
//     }
//
//     /// Insert [`RangeStyle`] into `Mask`. Internally it calls [`add`] method. See its
//     /// documentation.
//     ///
//     /// [`add`]: Mask::add
//     // pub fn insert(&mut self, (range, style): RangeStyle) {
//     //     self.add(range, style);
//     // }
//
//     /// Get `Style` for specified index `idx`. If no styles in this `Mask` for this `idx` then
//     /// default `Style` will be returned.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use yatui_text::*;
//     ///
//     /// let mask = Mask::new();
//     /// assert_eq!(mask.get(0), Style::default());
//     /// ```
//     ///
//     /// ```
//     /// # use yatui_text::*;
//     ///
//     /// let mut mask = Mask::new();
//     /// mask.add(0..=1, Style::new().fg(Color::Green));
//     /// assert_eq!(mask.get(0), Style::new().fg(Color::Green));
//     /// ```
//     ///
//     /// TODO: Add example with existed styles
//     pub fn get(&self, idx: usize) -> Style {
//         self.map.get(idx).cloned().unwrap_or_default()
//     }
//
//     /// Gets an iterator over all pairs of ranges and their styles. It returns non intersecting
//     /// ranges in ascending order with style info.
//     ///
//     /// The iterator element type is `(RangeInclusive<usize>, &'a Style)`.
//     pub fn iter(&self) -> Iter<'_> {
//         Iter { inner: self.map.iter() }
//     }
// }
//
// // impl std::iter::IntoIterator for Mask {
// //     type Item = RangeStyle;
// //     type IntoIter = IntoIter;
// //
// //     fn into_iter(self) -> Self::IntoIter {
// //         /// Gets an owned iterator over all pairs of ranges and their styles. It returns non
// //         /// intersecting ranges in ascending order with style info.
// //         Self::IntoIter { inner: self.map.into_iter() }
// //     }
// // }
//
// // fn convert_any_range_ref<S>(
// //     (range, style): (&btree_range_map::AnyRange<usize>, S),
// // ) -> (RangeInclusive<usize>, S) {
// //     (utils::bound_to_inclusive(*range), style)
// // }
//
// fn convert_any_range<S>(
//     (range, style): (btree_range_map::AnyRange<usize>, S),
// ) -> (RangeInclusive<usize>, S) {
//     convert_any_range_ref((&range, style))
// }
//
// impl<'a> Iterator for Iter<'a> {
//     type Item = (RangeInclusive<usize>, &'a Style);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next().map(convert_any_range_ref)
//     }
// }
//
// impl<'a> DoubleEndedIterator for Iter<'a> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.inner.next_back().map(convert_any_range_ref)
//     }
// }
//
// impl Iterator for IntoIter {
//     type Item = (RangeInclusive<usize>, Style);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.inner.next().map(convert_any_range)
//     }
// }
//
// impl DoubleEndedIterator for IntoIter {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.inner.next_back().map(convert_any_range)
//     }
// }
