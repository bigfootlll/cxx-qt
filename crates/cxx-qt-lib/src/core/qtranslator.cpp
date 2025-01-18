#include "cxx-qt-lib/qtranslator.h"

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
loadTranslation(QTranslator& translator, const QString& qmFilePath)
{
    return translator.load(qmFilePath);
}

}
}
