mod bindgen {
   pub struct A {
       a: u8,
       b: cxx::CxxString,
   }

   unsafe impl cxx::ExternType for A {
       type Id = cxx::type_id!("A");
       type Kind = cxx::kind::Trivial;
   }
   
}

#[cxx::bridge]
mod ffi {
   extern "C" {
       type A = crate::bindgen::A;

       fn get_a(a: A);
   }
}

fn main() {}
