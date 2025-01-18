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
            ptr: UniquePtr<QTranslator>,
            qmFilePath: &QString) -> bool;
    }
}

pub use ffi::QTranslator;

impl QTranslator {
    pub fn load_translation(ptr: cxx::UniquePtr<QTranslator>, qm_file_path: &str) -> bool {
        // 将 &str 转换为 &QString
        let qm_file_path = ffi::QString::from(qm_file_path);
        ffi::qtranslator_load_translation(ptr, &qm_file_path)
    }

    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::qtranslator_new()
    }
}
