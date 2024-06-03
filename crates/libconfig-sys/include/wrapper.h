#pragma once
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

using namespace libconfig;
