#include "cxx-qt-lib/qtranslator.h"
#include <QFile>
#include <QIODevice>
#include <QDebug>
#include <QDir>

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
    qDebug() << "loadTranslation: " << qmFilePath.toStdString().c_str();
    //list dir in resource 
    QDir dir(":/");
    QStringList files = dir.entryList(QDir::Files);
    qDebug() << "files: " << files.join(", ").toStdString().c_str();
    // load from resource 
    QFile file(qmFilePath);
    if (!file.open(QIODevice::ReadOnly)) {
        qDebug() << "Failed to open file: " << qmFilePath.toStdString().c_str();
        return false;
    }
    return translator->load(qmFilePath);
}

}
}
