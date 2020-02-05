#include "demo-cxx/demo.h"
#include "demo-rs/src/main.rs"
#include <iostream>

namespace org {
namespace rust {

ThingC::ThingC(std::string appname) : appname(std::move(appname)) {}

ThingC::~ThingC() { std::cout << "done with ThingC" << std::endl; }

std::unique_ptr<ThingC> make_demo(cxxbridge::RustStr appname) {
  return std::unique_ptr<ThingC>(new ThingC(appname));
}

const std::string &get_name(const ThingC &thing) { return thing.appname; }

std::unique_ptr<std::vector<uint8_t>> do_thing(SharedThing state) {
  print_r(*state.y);
  auto vec = std::unique_ptr<std::vector<uint8_t>>(new std::vector<uint8_t>());
  for (uint8_t i=0; i<10; i++) {
    vec->push_back(i*i);
  }
  return vec;
}


} // namespace rust
} // namespace org
