// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


trait noisy {
  fn speak(&mut self) -> int;
}

struct dog {
  barks: uint,

  volume: int,
}

impl dog {
    fn bark(&mut self) -> int {
      println!("Woof {} {}", self.barks, self.volume);
      self.barks += 1_usize;
      if self.barks % 3_usize == 0_usize {
          self.volume += 1;
      }
      if self.barks % 10_usize == 0_usize {
          self.volume -= 2;
      }
      println!("Grrr {} {}", self.barks, self.volume);
      self.volume
    }
}

impl noisy for dog {
    fn speak(&mut self) -> int {
        self.bark()
    }
}

fn dog() -> dog {
    dog {
        volume: 0,
        barks: 0_usize
    }
}

#[derive(Clone)]
struct cat {
  meows: uint,

  how_hungry: int,
  name: String,
}

impl noisy for cat {
    fn speak(&mut self) -> int {
        self.meow() as int
    }
}

impl cat {
    pub fn meow_count(&self) -> uint {
        self.meows
    }
}

impl cat {
    fn meow(&mut self) -> uint {
        println!("Meow");
        self.meows += 1_usize;
        if self.meows % 5_usize == 0_usize {
            self.how_hungry += 1;
        }
        self.meows
    }
}

fn cat(in_x: uint, in_y: int, in_name: String) -> cat {
    cat {
        meows: in_x,
        how_hungry: in_y,
        name: in_name
    }
}


fn annoy_neighbors(critter: &mut noisy) {
    for _i in 0_usize..10 { critter.speak(); }
}

pub fn main() {
  let mut nyan: cat = cat(0_usize, 2, "nyan".to_string());
  let mut whitefang: dog = dog();
  annoy_neighbors(&mut nyan);
  annoy_neighbors(&mut whitefang);
  assert_eq!(nyan.meow_count(), 10_usize);
  assert_eq!(whitefang.volume, 1);
}
