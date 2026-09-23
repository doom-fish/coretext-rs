use coretext::{
    AttributedString, CTFont, CTFrame, CTFramesetter, CTLine, CTRun, CTTypesetter, FontCollection,
    FontDescriptor, GlyphInfo, GlyphPath, MutableFontCollection, ParagraphStyle, RubyAnnotation,
    TextTab,
};

const fn assert_send_sync<T: Send + Sync>() {}

trait AmbiguousIfSend<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSend<()> for T {}
impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

trait AmbiguousIfSync<A> {
    fn check() {}
}
impl<T: ?Sized> AmbiguousIfSync<()> for T {}
impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

#[test]
fn immutable_font_objects_are_send_and_sync() {
    assert_send_sync::<CTFont>();
    assert_send_sync::<FontDescriptor>();
    assert_send_sync::<FontCollection>();
    assert_send_sync::<MutableFontCollection>();
    assert_send_sync::<GlyphInfo>();
    assert_send_sync::<GlyphPath>();
    assert_send_sync::<ParagraphStyle>();
    assert_send_sync::<TextTab>();
    assert_send_sync::<RubyAnnotation>();
    assert_send_sync::<AttributedString>();
}

#[test]
fn layout_objects_are_neither_send_nor_sync() {
    <CTTypesetter as AmbiguousIfSend<_>>::check();
    <CTTypesetter as AmbiguousIfSync<_>>::check();
    <CTFramesetter as AmbiguousIfSend<_>>::check();
    <CTFramesetter as AmbiguousIfSync<_>>::check();
    <CTFrame as AmbiguousIfSend<_>>::check();
    <CTFrame as AmbiguousIfSync<_>>::check();
    <CTLine as AmbiguousIfSend<_>>::check();
    <CTLine as AmbiguousIfSync<_>>::check();
    <CTRun as AmbiguousIfSend<_>>::check();
    <CTRun as AmbiguousIfSync<_>>::check();
}
