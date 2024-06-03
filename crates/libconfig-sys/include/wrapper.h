#pragma once
#include <libconfig.h++>
#include <memory>

#include "rust/cxx.h"

template <typename T, typename... Args>
auto construct_unique(Args... args) -> std::unique_ptr<T> {
  return std::make_unique<T>(args...);
}

using namespace libconfig;
