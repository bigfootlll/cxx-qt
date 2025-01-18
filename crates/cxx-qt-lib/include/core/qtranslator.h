#pragma once

#include <QCoreApplication>
#include <QTranslator>
#include <QString>
#include <memory>

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QTranslator> qtranslatorNew();

bool loadTranslation(QTranslator& translator, const QString &qmFilePath);

}
}
