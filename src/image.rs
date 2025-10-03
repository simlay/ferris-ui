use crate::View;
use objc2::rc::{PartialInit, Retained};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{NSObject, NSString};
use objc2_ui_kit::{UIImage, UIImageView, UIView};
use objc2_core_foundation::CGRect;

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
impl From<ImageType> for Option<Retained<UIImage>> {
    fn from(val: ImageType) -> Self {
        match val {
            ImageType::SystemIcon(icon_name) => unsafe {
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

        if let Some(image) = image {
            unsafe {
                let size = image.size();
                println!("IMAGE SIZE {size:?}");
                println!("IMAGE FRAME: {:?}", this.frame());
                println!("IMAGE BOUNDS: {:?}", this.bounds());
                let bounds = CGRect {
                    size: size,
                    ..Default::default()
                };
                this.setBounds(bounds);

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
    fn kind(&self) -> String {
        "Image".into()
    }
}
