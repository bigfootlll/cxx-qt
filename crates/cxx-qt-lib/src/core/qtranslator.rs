#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type QString = crate::QString;
        include!("cxx-qt-lib/common.h");
        include!("cxx-qt-lib/qtranslator.h");
        type QTranslator;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qtranslator_new"]
        fn qtranslatorNew() -> UniquePtr<QTranslator>;

        #[doc(hidden)]
        #[rust_name = "qtranslator_load_translation"]
        fn loadTranslation(
            ptr: Pin<&mut QTranslator>,
            qmFilePath: &QString) -> bool;
    }
}

pub use ffi::QTranslator;
use core::pin::Pin;

impl QTranslator {
    pub fn load_translation(ptr: Pin<&mut QTranslator>, qm_file_path: &ffi::QString) -> bool {
        ffi::qtranslator_load_translation(ptr, qm_file_path)
    }

    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qtranslator_new()
    }
}
