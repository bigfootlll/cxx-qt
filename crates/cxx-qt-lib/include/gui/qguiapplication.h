// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtGui/QFont>
#include <QtGui/QGuiApplication>
#include <QtCore/QTranslator>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args);

void
qguiapplicationSetFont(QGuiApplication& app, const QFont& font);

QFont
qguiapplicationFont(const QGuiApplication& app);

void
qguiapplicationSetDesktopFileName(const QString& name);

QString
qguiapplicationDesktopFileName();

void qguiapplicationSetupLanguage(const QString& language);

template<typename T>
bool
qguiapplicationLoadTranslation(T& app, const QString& qmFilePath) {
  // Create translator as a raw pointer since QGuiApplication takes ownership
  auto* translator = new QTranslator();
  if (!translator->load(qmFilePath)) {
    qDebug() << "Failed to load translation file:" << qmFilePath;
    delete translator;
    return false;
  } else {
    qDebug() << "Loaded translation file:" << qmFilePath;
  }
  // QGuiApplication takes ownership of the translator
  return app.installTranslator(translator);
}

}
}
