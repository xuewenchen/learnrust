use mylib::fatory::machine;
use mylib::fatory::machine as A;


pub mod person {
    pub mod human {
        #[derive(Debug)]
        pub struct Man {
            pub age: u32,
            name: String,
        }

        impl Man {
            pub fn new_man() -> Man {
                Man{
                    age: 12,
                    name: String::from("chenxuewen"),
                }
            }

            pub fn echo(&self) {
                println!("age = {}, name = {}", self.age, self.name);
            }
        }
    }

    pub mod ModA {
        pub fn echo() {
            println!("ModA");
        }
        
        pub mod ModB {
            pub fn echo() {
                println!("ModB");
                super::echo();
            }
        }
    }
}


fn main() {
    // use
    machine::person::echo(); 

    // use
    A::person::echo(); 

    // not use use
    mylib::fatory::machine::person::echo(); 

    // use mod struct
    let a = person::human::Man::new_man();
    a.echo();
    println!("a = {:#?}", a);

    person::ModA::ModB::echo();
}
