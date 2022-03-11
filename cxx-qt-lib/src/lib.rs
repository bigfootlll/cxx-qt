// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod update_requester;
pub use update_requester::UpdateRequestHandler;

mod qcolor;
pub use qcolor::{Color, QColor};

mod qdate;
pub use qdate::QDate;

mod qdatetime;
pub use qdatetime::{DateTime, QDateTime};

mod qrect;
pub use qrect::QRect;

mod qrectf;
pub use qrectf::QRectF;

mod qsize;
pub use qsize::QSize;

mod qsizef;
pub use qsizef::QSizeF;

mod qstring;
pub use qstring::QString;

mod qtime;
pub use qtime::QTime;

mod qpoint;
pub use qpoint::QPoint;

mod qpointf;
pub use qpointf::QPointF;

mod qurl;
pub use qurl::{QUrl, Url};

mod qvariant;
pub use qvariant::{QVariant, Variant, VariantValue};

mod map_qt_value;
pub use map_qt_value::*;

// Provide a separate depending on the platform
// this is because include_str requires th correct and non-mixed path separators
//
// https://github.com/rust-lang/rust/issues/75075
#[cfg(not(windows))]
macro_rules! sep {
    () => {
        "/"
    };
}

#[cfg(windows)]
macro_rules! sep {
    () => {
        "\\"
    };
}

pub const QT_TYPES_HEADER: &str =
    include_str!(concat!("..", sep!(), "include", sep!(), "qt_types.h"));
pub const QT_TYPES_SOURCE: &str = include_str!("qt_types.cpp");

pub const QCOLOR_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qcolor_cxx.h"
));
pub const QCOLOR_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qcolor_cxx.cpp"
));
pub const QDATE_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qdate_cxx.h"
));
pub const QDATE_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qdate_cxx.cpp"
));
pub const QDATETIME_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qdatetime_cxx.h"
));
pub const QDATETIME_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qdatetime_cxx.cpp"
));
pub const QPOINT_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qpoint_cxx.h"
));
pub const QPOINT_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qpoint_cxx.cpp"
));
pub const QPOINTF_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qpointf_cxx.h"
));
pub const QPOINTF_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qpointf_cxx.cpp"
));
pub const QRECT_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qrect_cxx.h"
));
pub const QRECT_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qrect_cxx.cpp"
));
pub const QRECTF_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qrectf_cxx.h"
));
pub const QRECTF_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qrectf_cxx.cpp"
));
pub const QSIZE_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qsize_cxx.h"
));
pub const QSIZE_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qsize_cxx.cpp"
));
pub const QSIZEF_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qsizef_cxx.h"
));
pub const QSIZEF_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qsizef_cxx.cpp"
));
pub const QSTRING_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qstring_cxx.h"
));
pub const QSTRING_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qstring_cxx.cpp"
));
pub const QTIME_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qtime_cxx.h"
));
pub const QTIME_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qtime_cxx.cpp"
));
pub const QURL_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qurl_cxx.h"
));
pub const QURL_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qurl_cxx.cpp"
));
pub const QVARIANT_CXX_HEADER: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "include",
    sep!(),
    "qvariant_cxx.h"
));
pub const QVARIANT_CXX_SOURCE: &str = include_str!(concat!(
    env!("OUT_DIR"),
    sep!(),
    "cxx-qt-lib",
    sep!(),
    "src",
    sep!(),
    "qvariant_cxx.cpp"
));

pub trait PropertyChangeHandler<C, P> {
    fn handle_property_change(&mut self, cpp: &mut C, property: P);
}

pub trait ToUniquePtr {
    type CppType;

    fn to_unique_ptr(self) -> cxx::UniquePtr<Self::CppType>
    where
        Self::CppType: cxx::memory::UniquePtrTarget;
}
