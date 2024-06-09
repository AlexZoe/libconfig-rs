#pragma once
#include <cstdint>
#include <libconfig.h++>
#include <memory>
#include <string>

#include "rust/cxx.h"

template <typename T, typename... Args>
auto construct_unique(Args... args) -> std::unique_ptr<T> {
  return std::make_unique<T>(args...);
}

namespace rust {
namespace behavior {

template <typename Try, typename Fail>
static void trycatch(Try &&func, Fail &&fail) noexcept try {
  func();
} catch (const libconfig::ParseException &pex) {
  fail("Parse error at " + std::string(pex.getFile()) + ":" +
       std::to_string(pex.getLine()) + std::string(" - ") +
       std::string(pex.getError()));
} catch (const std::exception &e) {
  fail(e.what());
}

}  // namespace behavior
}  // namespace rust

namespace libconfig {

auto lookupSettingFromSetting(Setting &setting, const char *path) -> Setting & {
  return setting.lookup(path);
}

auto lookupValueU64FromSetting(const Setting &config, const char *path,
                               uint64_t &value) -> bool {
  unsigned long long tmp;
  if (config.lookupValue(path, tmp)) {
    value = tmp;
    return true;
  }
  return false;
}

auto lookupValueI64FromSetting(const Setting &setting, const char *path,
                               int64_t &value) -> bool {
  long long tmp;
  if (setting.lookupValue(path, tmp)) {
    value = tmp;
    return true;
  }
  return false;
}

auto getPathFromSetting(const Setting &setting, std::string &path) -> void {
  path = setting.getPath();
}

auto getRootFromConfig(const Config &config) -> Setting & {
  return config.getRoot();
}

auto lookupSettingFromConfig(Config &config, const char *path) -> Setting & {
  return config.lookup(path);
}

auto lookupValueU64FromConfig(const Config &config, const char *path,
                              uint64_t &value) -> bool {
  unsigned long long tmp;
  if (config.lookupValue(path, tmp)) {
    value = tmp;
    return true;
  }
  return false;
}

auto lookupValueI64FromConfig(const Config &config, const char *path,
                              int64_t &value) -> bool {
  long long tmp;
  if (config.lookupValue(path, tmp)) {
    value = tmp;
    return true;
  }
  return false;
}

}  // namespace libconfig

using namespace libconfig;
