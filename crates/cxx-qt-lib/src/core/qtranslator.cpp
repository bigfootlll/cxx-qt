#include "cxx-qt-lib/qtranslator.h"

namespace rust {
namespace cxxqtlib1 {

static QTranslator g_translator;

bool
loadTranslation(const QString& qmFilePath)
{
  // 尝试加载指定的翻译文件
  if (g_translator.load(qmFilePath)) {
    QCoreApplication::installTranslator(&g_translator);
    return true;
  } else {
    return false;
  }
}
}
}
