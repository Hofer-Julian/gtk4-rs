// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use TextBuffer;
use TextChildAnchor;
use TextMark;
use TextSearchFlags;
use TextTag;
use gdk;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use pango;
use std::cmp;

glib_wrapper! {
    #[derive(Debug, Hash)]
    pub struct TextIter(Boxed<gtk_sys::GtkTextIter>);

    match fn {
        copy => |ptr| gtk_sys::gtk_text_iter_copy(mut_override(ptr)),
        free => |ptr| gtk_sys::gtk_text_iter_free(ptr),
        get_type => || gtk_sys::gtk_text_iter_get_type(),
    }
}

impl TextIter {
    pub fn assign(&mut self, other: &TextIter) {
        unsafe {
            gtk_sys::gtk_text_iter_assign(self.to_glib_none_mut().0, other.to_glib_none().0);
        }
    }

    pub fn backward_char(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_char(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_chars(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_find_char<P: FnMut(char) -> bool>(&mut self, pred: P, limit: Option<&TextIter>) -> bool {
        let pred_data: P = pred;
        unsafe extern "C" fn pred_func<P: FnMut(char) -> bool>(ch: u32, user_data: glib_sys::gpointer) -> glib_sys::gboolean {
            let ch = from_glib(ch);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(ch);
            res.to_glib()
        }
        let pred = Some(pred_func::<P> as _);
        let super_callback0: &P = &pred_data;
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_find_char(self.to_glib_none_mut().0, pred, super_callback0 as *const _ as usize as *mut _, limit.to_glib_none().0))
        }
    }

    pub fn backward_line(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_line(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_search(&self, str: &str, flags: TextSearchFlags, limit: Option<&TextIter>) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_text_iter_backward_search(self.to_glib_none().0, str.to_glib_none().0, flags.to_glib(), match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, limit.to_glib_none().0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    pub fn backward_sentence_start(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_sentence_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_sentence_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_sentence_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_to_tag_toggle<P: IsA<TextTag>>(&mut self, tag: Option<&P>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_to_tag_toggle(self.to_glib_none_mut().0, tag.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    pub fn backward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_line(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_visible_word_start(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_word_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_visible_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_visible_word_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn backward_word_start(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_word_start(self.to_glib_none_mut().0))
        }
    }

    pub fn backward_word_starts(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_backward_word_starts(self.to_glib_none_mut().0, count))
        }
    }

    pub fn can_insert(&self, default_editability: bool) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_can_insert(self.to_glib_none().0, default_editability.to_glib()))
        }
    }

    fn compare(&self, rhs: &TextIter) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_compare(self.to_glib_none().0, rhs.to_glib_none().0)
        }
    }

    pub fn editable(&self, default_setting: bool) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_editable(self.to_glib_none().0, default_setting.to_glib()))
        }
    }

    pub fn ends_line(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_ends_line(self.to_glib_none().0))
        }
    }

    pub fn ends_sentence(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_ends_sentence(self.to_glib_none().0))
        }
    }

    pub fn ends_tag<P: IsA<TextTag>>(&self, tag: Option<&P>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_ends_tag(self.to_glib_none().0, tag.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    pub fn ends_word(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_ends_word(self.to_glib_none().0))
        }
    }

    fn equal(&self, rhs: &TextIter) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_equal(self.to_glib_none().0, rhs.to_glib_none().0))
        }
    }

    pub fn forward_char(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_char(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_chars(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_chars(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_find_char<P: FnMut(char) -> bool>(&mut self, pred: P, limit: Option<&TextIter>) -> bool {
        let pred_data: P = pred;
        unsafe extern "C" fn pred_func<P: FnMut(char) -> bool>(ch: u32, user_data: glib_sys::gpointer) -> glib_sys::gboolean {
            let ch = from_glib(ch);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            let res = (*callback)(ch);
            res.to_glib()
        }
        let pred = Some(pred_func::<P> as _);
        let super_callback0: &P = &pred_data;
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_find_char(self.to_glib_none_mut().0, pred, super_callback0 as *const _ as usize as *mut _, limit.to_glib_none().0))
        }
    }

    pub fn forward_line(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_line(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_search(&self, str: &str, flags: TextSearchFlags, limit: Option<&TextIter>) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut match_start = TextIter::uninitialized();
            let mut match_end = TextIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_text_iter_forward_search(self.to_glib_none().0, str.to_glib_none().0, flags.to_glib(), match_start.to_glib_none_mut().0, match_end.to_glib_none_mut().0, limit.to_glib_none().0));
            if ret { Some((match_start, match_end)) } else { None }
        }
    }

    pub fn forward_sentence_end(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_sentence_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_sentence_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_sentence_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_to_end(&mut self) {
        unsafe {
            gtk_sys::gtk_text_iter_forward_to_end(self.to_glib_none_mut().0);
        }
    }

    pub fn forward_to_line_end(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_to_line_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_to_tag_toggle<P: IsA<TextTag>>(&mut self, tag: Option<&P>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_to_tag_toggle(self.to_glib_none_mut().0, tag.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    pub fn forward_visible_cursor_position(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_cursor_position(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_cursor_positions(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_cursor_positions(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_visible_line(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_line(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_lines(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_lines(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_visible_word_end(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_word_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_visible_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_visible_word_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn forward_word_end(&mut self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_word_end(self.to_glib_none_mut().0))
        }
    }

    pub fn forward_word_ends(&mut self, count: i32) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_forward_word_ends(self.to_glib_none_mut().0, count))
        }
    }

    pub fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_iter_get_buffer(self.to_glib_none().0))
        }
    }

    pub fn get_bytes_in_line(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_bytes_in_line(self.to_glib_none().0)
        }
    }

    pub fn get_char(&self) -> char {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_get_char(self.to_glib_none().0))
        }
    }

    pub fn get_chars_in_line(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_chars_in_line(self.to_glib_none().0)
        }
    }

    pub fn get_child_anchor(&self) -> Option<TextChildAnchor> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_iter_get_child_anchor(self.to_glib_none().0))
        }
    }

    pub fn get_language(&self) -> Option<pango::Language> {
        unsafe {
            from_glib_full(gtk_sys::gtk_text_iter_get_language(self.to_glib_none().0))
        }
    }

    pub fn get_line(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_line(self.to_glib_none().0)
        }
    }

    pub fn get_line_index(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_line_index(self.to_glib_none().0)
        }
    }

    pub fn get_line_offset(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_line_offset(self.to_glib_none().0)
        }
    }

    pub fn get_marks(&self) -> Vec<TextMark> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_text_iter_get_marks(self.to_glib_none().0))
        }
    }

    pub fn get_offset(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_offset(self.to_glib_none().0)
        }
    }

    pub fn get_slice(&self, end: &TextIter) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_text_iter_get_slice(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_tags(&self) -> Vec<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_text_iter_get_tags(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self, end: &TextIter) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_text_iter_get_text(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_texture(&self) -> Option<gdk::Texture> {
        unsafe {
            from_glib_none(gtk_sys::gtk_text_iter_get_texture(self.to_glib_none().0))
        }
    }

    pub fn get_toggled_tags(&self, toggled_on: bool) -> Vec<TextTag> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gtk_sys::gtk_text_iter_get_toggled_tags(self.to_glib_none().0, toggled_on.to_glib()))
        }
    }

    pub fn get_visible_line_index(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_visible_line_index(self.to_glib_none().0)
        }
    }

    pub fn get_visible_line_offset(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_text_iter_get_visible_line_offset(self.to_glib_none().0)
        }
    }

    pub fn get_visible_slice(&self, end: &TextIter) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_text_iter_get_visible_slice(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn get_visible_text(&self, end: &TextIter) -> Option<GString> {
        unsafe {
            from_glib_full(gtk_sys::gtk_text_iter_get_visible_text(self.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn has_tag<P: IsA<TextTag>>(&self, tag: &P) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_has_tag(self.to_glib_none().0, tag.as_ref().to_glib_none().0))
        }
    }

    pub fn in_range(&self, start: &TextIter, end: &TextIter) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_in_range(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0))
        }
    }

    pub fn inside_sentence(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_inside_sentence(self.to_glib_none().0))
        }
    }

    pub fn inside_word(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_inside_word(self.to_glib_none().0))
        }
    }

    pub fn is_cursor_position(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_is_cursor_position(self.to_glib_none().0))
        }
    }

    pub fn is_end(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_is_end(self.to_glib_none().0))
        }
    }

    pub fn is_start(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_is_start(self.to_glib_none().0))
        }
    }

    pub fn order(&mut self, second: &mut TextIter) {
        unsafe {
            gtk_sys::gtk_text_iter_order(self.to_glib_none_mut().0, second.to_glib_none_mut().0);
        }
    }

    pub fn set_line(&mut self, line_number: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_line(self.to_glib_none_mut().0, line_number);
        }
    }

    pub fn set_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    pub fn set_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    pub fn set_offset(&mut self, char_offset: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_offset(self.to_glib_none_mut().0, char_offset);
        }
    }

    pub fn set_visible_line_index(&mut self, byte_on_line: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_visible_line_index(self.to_glib_none_mut().0, byte_on_line);
        }
    }

    pub fn set_visible_line_offset(&mut self, char_on_line: i32) {
        unsafe {
            gtk_sys::gtk_text_iter_set_visible_line_offset(self.to_glib_none_mut().0, char_on_line);
        }
    }

    pub fn starts_line(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_starts_line(self.to_glib_none().0))
        }
    }

    pub fn starts_sentence(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_starts_sentence(self.to_glib_none().0))
        }
    }

    pub fn starts_tag<P: IsA<TextTag>>(&self, tag: Option<&P>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_starts_tag(self.to_glib_none().0, tag.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    pub fn starts_word(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_starts_word(self.to_glib_none().0))
        }
    }

    pub fn toggles_tag<P: IsA<TextTag>>(&self, tag: Option<&P>) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_text_iter_toggles_tag(self.to_glib_none().0, tag.map(|p| p.as_ref()).to_glib_none().0))
        }
    }
}

impl PartialOrd for TextIter {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TextIter {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl PartialEq for TextIter {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for TextIter {}
