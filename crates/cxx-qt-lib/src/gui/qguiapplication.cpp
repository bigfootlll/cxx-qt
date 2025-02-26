// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qguiapplication.h"

#include "cxx-qt-lib/qcoreapplication.h"
#include <QTranslator>
#include <QDebug>
#include <QEvent>
#include "cxx-qt-lib/qqmlapplicationengine.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QGuiApplication
  // by storing it inside a QObject that has QGuiApplication as it's parent
  auto argsData = new ApplicationArgsData(args);
  // Note that QGuiApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QGuiApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}

void
qguiapplicationSetFont(QGuiApplication& app, const QFont& font)
{
  app.setFont(font);
}

QFont
qguiapplicationFont(const QGuiApplication& app)
{
  return app.font();
}

void
qguiapplicationSetDesktopFileName(const QString& name)
{
  QGuiApplication::setDesktopFileName(name);
}

QString
qguiapplicationDesktopFileName()
{
  return QGuiApplication::desktopFileName();
}

void
qguiapplicationSetupLanguage(const QString& language)
{
  auto* translator = new QTranslator();
  if (!translator->load(language)) {
    qDebug() << "Failed to load translation file:" << language;
    delete translator;
    return;
  } else {
    qDebug() << "Loaded translation file:" << language;
  }
  // QGuiApplication takes ownership of the translator
  QGuiApplication::installTranslator(translator);
  QEvent languageChangeEvent(QEvent::LanguageChange);
  QGuiApplication::sendEvent(QGuiApplication::instance(), &languageChangeEvent);

  // 遍历所有子对象查找 QQmlApplicationEngine
  auto engines = QGuiApplication::instance()->findChildren<QQmlApplicationEngine*>();
  qDebug() << "Found" << engines.size() << "QQmlApplicationEngine objects";
  for (auto* engine : engines) {
    qDebug() << "Retranslating QQmlApplicationEngine:" << engine;
    engine->retranslate();
  }
}
}
}
