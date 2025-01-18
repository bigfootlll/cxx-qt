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
        #[rust_name = "qtranslator_load_translation"]
        fn loadTranslation(qmFilePath: &QString) -> bool;
    }
}

pub use ffi::QTranslator;

impl QTranslator {
    pub fn load_translation(&self, qm_file_path: &ffi::QString) -> bool {
        ffi::qtranslator_load_translation(qm_file_path)
    }
}

