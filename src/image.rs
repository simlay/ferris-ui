use crate::View;
use objc2::rc::{PartialInit, Retained};
use objc2::{AllocAnyThread, MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_core_foundation::{CGPoint, CGRect};
use objc2_foundation::{NSObject, NSString};
use objc2_ui_kit::{UIEdgeInsets, UIImage, UIImageView, UILabel, UIView};

define_class!(
    #[unsafe(super(UIImageView, UIView, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisUIImage"]
    pub struct Image;

    impl Image { }
);

pub enum ImageType {
    SystemIcon(String),
}
impl Into<Option<Retained<UIImage>>> for ImageType {
    fn into(self) -> Option<Retained<UIImage>> {
        match self {
            Self::SystemIcon(icon_name) => unsafe {
                UIImage::systemImageNamed(&NSString::from_str(icon_name.as_str()))
            },
        }
    }
}

impl Image {
    pub fn new<T: Into<Option<Retained<UIImage>>>>(
        mtm: MainThreadMarker,
        image: T,
    ) -> Retained<Self> {
        let this: PartialInit<Self> = mtm.alloc().set_ivars(());
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        let image: Option<Retained<UIImage>> = image.into();

        unsafe {
            if let Some(image) = image {
                let size = image.size();
                println!("IMAGE SIZE {size:?}");
                println!("FRAME: {:?}", this.frame());
                println!("BOUNDS: {:?}", this.bounds());
                this.setImage(Some(&image));
            }
        }

        this
    }
}
impl View for Image {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
}
