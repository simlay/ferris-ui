use crate::View;
use objc2::rc::{
    Retained,
    PartialInit,
};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{NSObject, NSString};
use objc2_ui_kit::{UIImageView, UIImage, UILabel, UIView};

define_class!(
    #[unsafe(super(UIImageView, UIView, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisUIImage"]
    pub struct Image;

    impl Image { }
);

trait IntoImage: Sized {
    fn into_image(self) -> UIImage;
}
pub enum ImageType {
    SystemIcon(String),
}

impl IntoImage for ImageType {
    fn into_image(self) -> UIImage {
        todo!();
    }
}

impl Image {
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this : PartialInit<Self>= mtm.alloc().set_ivars(());
        let this : Retained<Self> = unsafe { msg_send![super(this), init] };
        this
    }

    pub fn set_image<T: IntoImage>(&self, new_image: T) {
        // UNANSWERED: Is this safe?
        unsafe {
            self.setImage(Some(&new_image.into_image()))
        }
    }

    pub fn with_text<T: IntoImage>(self: Retained<Self>, new_text: T) -> Retained<Self> {
        self.set_image(new_text);
        self
    }

    pub fn clear_text(&self) {
    }
}
impl View for Image {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
}
