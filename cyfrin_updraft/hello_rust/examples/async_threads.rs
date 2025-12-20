#![allow(unused)]

// use std::time::Duration;
use std::time::{Duration, Instant};
use tokio::join;
use tokio::task::JoinHandle;

struct Tomato;
struct Lettuce;
struct Cheese;
struct Patty;
struct Bun;

struct Hamburger {
    pub tomato: Tomato,
    pub lettuce: Lettuce,
    pub cheese: Cheese,
    pub patty: Patty,
    pub bun: Bun,
}

async fn toast_bun() -> Bun { Bun}

async fn cook_patty() -> Patty { Patty}

async fn get_veggies() -> (Tomato, Lettuce) {(Tomato, Lettuce)}

async fn get_cheese() -> Cheese {Cheese }

async fn make_hamburger_seq() -> Hamburger {
    let bun = toast_bun().await;
    let patty = cook_patty().await;
    let (tomato, lettuce) = get_veggies().await;
    let cheese = get_cheese().await;

    println!("is ready");

    Hamburger{
        tomato,
        lettuce,
        cheese,
        patty,
        bun
    }
}

async fn make_hamburger() -> Hamburger {
    let (bun, patty, (tomato, lettuce), cheese ) =  tokio::join!{
        toast_bun(), 
        cook_patty(),
        get_veggies(),
        get_cheese()
    };

    println!("is ready");

    Hamburger{
        tomato,
        lettuce,
        cheese,
        patty,
        bun
    }
}

//When to use threads vs async / await ? 
#[tokio::main]
async fn main() {
   let fut = make_hamburger();
  // fut.await;

   let hamburger = make_hamburger().await;

   // Spawning too many threads can crash this program (OS thread and memory limits)
//    let mut handles = vec![];
//    for i in 0..1000000 {
//     handles.push(std::thread::spawn(move || {
//         std::thread::sleep(Duration::from_millis(100));
//         println!("{i}: is ready");
//     }));
//    }

//    for h in handles {
//     h.join().unwrap();
//    }

   let mut handles = vec![];
   for i in 0..1000000 {

    let fut = async move  {
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("{i}: is ready");
    };

    //let handler : tokio::task::JoinHandle<()> = tokio::task::spawn(fut);
    let handler: JoinHandle<()> = tokio::task::spawn(fut);
    handles.push(handler);
    // handles.push(std::thread::spawn(move || {
    //     std::thread::sleep(Duration::from_millis(100));
    //     println!("{i}: is ready");
    // }));
   }

   for h in handles {
    h.await.unwrap();
   }



}