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
  for (uint8_t i = 0; i < 10; i++) {
    vec->push_back(i * i);
  }
  return vec;
}

JsonBlob get_jb(const cxxbridge::RustVec<uint8_t>& vec) {
  JsonBlob retval;

  std::cout << "incoming vec length is " << vec.size() << "\n";
  auto vec_copy = std::vector<uint8_t>();
  vec_copy.reserve(vec.length());
  vec.to_vector(vec_copy);
  std::cout << "vec_copy length is " << vec_copy.size() << "\n";
  std::cout << "vec_copy[0] is " << (int)vec_copy[0] << "\n";

  auto blob = std::unique_ptr<std::vector<uint8_t>>(new std::vector<uint8_t>());
  for (uint8_t i = 0; i < 10; i++) {
    blob->push_back(i * 2);
  }

  auto json = std::unique_ptr<std::string>(new std::string("{\"demo\": 23}"));

  retval.json = std::move(json);
  retval.blob = std::move(blob);

  return retval;
}

}  // namespace rust
}  // namespace org
