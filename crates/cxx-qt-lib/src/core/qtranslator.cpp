#include "cxx-qt-lib/qtranslator.h"
#include <QFile>
#include <QIODevice>
#include <QDebug>
#include <QDir>
#include <QStringList>
namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QTranslator>
qtranslatorNew()
{
    auto translator = ::std::make_unique<QTranslator>();
    QCoreApplication::installTranslator(translator.get());
    return translator;
}

bool
loadTranslation(std::unique_ptr<QTranslator> translator, const QString& qmFilePath)
{
    if (!translator->load(qmFilePath)) {
        qDebug() << "Failed to load translation file:" << qmFilePath;
        return false;
    }
    
    // Make sure the translator is installed
    if (!QCoreApplication::installTranslator(translator.get())) {
        qDebug() << "Failed to install translator";
        return false;
    }
    
    return true;
}
}
}
