use objc2::rc::Retained;
use objc2::AllocAnyThread;
use objc2_core_foundation::CGSize;
use objc2_foundation::{
    NSSearchPathDirectory, NSSearchPathDomainMask, NSSearchPathForDirectoriesInDomains,
};
use objc2_ui_kit::{
    UIGraphicsBeginImageContext,
    //UIGraphicsGetCurrentContext,
    UIGraphicsEndImageContext,
    UIGraphicsGetImageFromCurrentImageContext,
    UIGraphicsImageRenderer,
    //UIGraphicsImageRendererContext,
    UIImage,
    UIImagePNGRepresentation,
    //UIImageWriteToSavedPhotosAlbum,
};
use std::fs::write;
pub fn save_image(image: Retained<UIImage>) {
    let path = unsafe {
        NSSearchPathForDirectoriesInDomains(
            NSSearchPathDirectory::DocumentDirectory,
            NSSearchPathDomainMask::UserDomainMask,
            true,
        )
    };
    let data = unsafe { UIImagePNGRepresentation(&image) }
        .unwrap()
        .to_vec();
    println!("DATA IS {:?}", data.len());

    if let Some(path) = path.firstObject() {
        let path = path.to_string();
        let path = std::path::Path::new(&path).join("foobar.png");
        println!("PATH IS {path:?}");
        //let mut output = File::create(path).unwrap();
        let _ = write(path, data);
    }
}
pub fn path(ns_search_path: NSSearchPathDirectory) -> String {
    let path = unsafe {
        NSSearchPathForDirectoriesInDomains(
            ns_search_path,
            NSSearchPathDomainMask::UserDomainMask,
            true,
        )
    };
    let out = path.firstObject();
    out.map(|i| i.to_string()).unwrap_or_default()
}
pub fn take_screenshot(size: CGSize) -> Option<Retained<UIImage>> {
    println!("Taking screenshot at size: {size:?}");
    unsafe { UIGraphicsBeginImageContext(size) };
    let _renderer =
        unsafe { UIGraphicsImageRenderer::initWithSize(UIGraphicsImageRenderer::alloc(), size) };
    let image = unsafe { UIGraphicsGetImageFromCurrentImageContext() };
    //let path = NSSearchPathDirectory::PicturesDirectory;
    unsafe { UIGraphicsEndImageContext() };
    image
}
