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

typedef libconfig::Setting::Type LibType;

namespace libconfig {

auto setBool(Setting &setting, bool val) { setting = val; }
auto setI32(Setting &setting, int32_t val) { setting = val; }
auto setI64(Setting &setting, int64_t val) { setting = val; }
auto setF32(Setting &setting, float val) { setting = val; }
auto setF64(Setting &setting, double val) { setting = val; }
auto setString(Setting &setting, const std::string &val) { setting = val; }

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

auto addSetting(Setting &setting, const std::string &name, Setting::Type type)
    -> Setting & {
  return setting.add(name, type);
}

auto tryBoolFromSetting(const Setting &setting) -> bool { return setting; }
auto tryI32FromSetting(const Setting &setting) -> int32_t { return setting; }
auto tryI64FromSetting(const Setting &setting) -> int64_t { return setting; }
auto tryF32FromSetting(const Setting &setting) -> float { return setting; }
auto tryF64FromSetting(const Setting &setting) -> double { return setting; }
auto tryStringFromSetting(const Setting &setting)
    -> std::unique_ptr<std::string> {
  return std::make_unique<std::string>(static_cast<const char *>(setting));
}

auto getPathFromSetting(const Setting &setting, std::string &path) -> void {
  path = setting.getPath();
}

auto getParentFromSetting(Setting &setting) -> Setting & {
  return setting.getParent();
}

auto getSettingIter(Setting &setting) -> std::unique_ptr<SettingIterator> {
  return std::make_unique<SettingIterator>(setting.begin());
}

auto getNextFromIter(std::unique_ptr<SettingIterator> &iter) -> Setting & {
  return (*(*iter)++);
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
