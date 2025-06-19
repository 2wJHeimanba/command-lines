use std::{marker::PhantomPinned, pin::Pin, time::Duration};

use command_lines::Result;
use tokio::{pin, time::sleep};

async fn test_fn() {
  let name = String::from("fdsafds");
  let name_ref = &name;
  sleep(Duration::from_secs(3)).await;
  println!("name: {name_ref}");
}

struct FnStruct {
  name: String,
}
impl Future for FnStruct {
  type Output = String;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    ::std::task::Poll::Ready(self.name.clone())
  }
}

#[derive(Debug, Clone)]
struct Person {
  name: String,
  age: u8,
  _pin: PhantomPinned,
}

impl Future for Person {
  type Output = String;
  fn poll(
    self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    ::std::task::Poll::Ready(self.name.clone())
  }
}

fn test_fn1<T: Unpin>(t: T) {}

struct TestStruct {
  name: String,
}
// impl TestStruct {
//   fn test(self: Option<&mut Self>) {}
// }

fn test_pin<T: Unpin>(params: T) {}

struct Table {
  _pin: PhantomPinned,
}
impl Future for Table {
  type Output = String;
  fn poll(
    self: Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    todo!()
  }
}

struct Student {
  name: String,
}
impl Student {
  fn set_name(self: &mut Self) {
    self.name.push_str("fdsafds¡");
  }
  fn get_name(self: Box<&mut Self>) {
    let res = &self.name;
    println!("res: {res}");
    // match self {
    //   Some(val) => {
    //     println!("{}", val.name);
    //   }
    //   None => {
    //     println!("no student");
    //   }
    // }
  }
}

async fn test_async() {}

fn test_future<T: Future<Output = ()>>(params: T) {}

#[tokio::main]
async fn main() -> Result<()> {
  let mut student = Student {
    name: "fuck tcl".into(),
  };
  let box_student = Box::new(&mut student);
  box_student.get_name();

  let name = String::from("vans");
  let box_name = Box::new(&name);
  let name_prt = &name as *const String;
  unsafe {
    let res = &*name_prt;
    println!("{res}");
  }
  let box_name_ptr = &box_name as *const Box<&String>;
  println!("{name_prt:p}");
  // let res = *box_name_ptr;
  println!("{box_name_ptr:p}");
  println!("{box_name:p}");

  let res = Box::into_raw(box_name);
  unsafe {
    let res = &**res;
    println!("--{res}");
  }
  println!("{res:p}");

  let name = String::from("vans");
  let pin_name = Pin::new(&name);
  let temp_name = pin_name;
  // *pin_name;
  // name;

  let name = Box::new(String::from("vans"));
  let mut table = Table {
    _pin: PhantomPinned,
  };
  let pin_table = Box::pin(&mut table);
  // let temp_table = pin_table;
  let test_async_fn = Box::pin(test_async());

  unsafe {
    let mut res = test_async();
    // let res = Pin::new_unchecked(&res);
    let res = Box::pin(res);
    // res.poll(cx)
    test_future(res);
  };

  // Pin::new(test_async());
  // test_async_fn
  // test_pin(table);
  test_pin(test_async_fn);
  test_pin(name);

  let fu_test = FnStruct {
    name: "wenjianjia".into(),
  };
  // fu_test
  let res = fu_test.await;
  dbg!(res);

  let mut person = Person {
    name: "vans".into(),
    age: 12,
    _pin: PhantomPinned,
  };

  // let person1 = test_fn1(23);
  // let result = person.clone().await;
  // let result = person.clone().await;

  // let result = Box::pin(person);
  // let res = result.await;
  // dbg!(result);
  // test_fn().await;

  let res = async { 23 };

  let result = unsafe { Pin::new_unchecked(&mut person) };

  // Person::poll(result, cx);

  // let name = String::from("fdsafds");
  // let pin_name = Pin::new(name);

  let mut val: u8 = 5;
  let mut pinned: Pin<&mut u8> = Pin::new(&mut val);
  println!("{:p}", pinned); // 5
  pinned.as_mut().set(10);
  println!("{:p}", pinned); // 10

  Ok(())
}
