use master_async::*;
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    // fndmntls::do_task().await
    // fndmntls::use_ready_fut()

    //join awaits multiple futures concurrently o the same task
    let (_, _) = tokio::join!(fndmntls::do_task(), fndmntls::use_ready_fut(),);

    //join with biased
    //control polling order
    let (_first, _second) = tokio::join!(
        biased;
        fndmntls::do_task(), fndmntls::use_ready_fut(),);

    //try_join! will fail fast if theres any error
    // let res = tokio::try_join!(fndmntls::do_task(), fndmntls::use_ready_fut());
    // match res {
    //     Ok((first, second)) => {
    //         //
    //     }

    //     Err(err) => {
    //         println!("error");
    //     }
    // }

    //using pin!
    let future = fndmntls::do_task();
    tokio::pin!(future);
    (&mut future).await;

    //using pin! + select!
    let mut stream = stream::iter(vec![1, 2, 3, 4]);

    let fut = fndmntls::do_task();
    tokio::pin!(fut);

    loop {
        tokio::select! {
            _ = &mut fut =>{
                //stop looping fut will be polled after completion
                break;
            }

            Some(val)= stream.next()=>{
                println!("got value = {}", val);
            }
        }
    }

    //pin + select variant
    tokio::pin! {
        let fut1 = fndmntls::do_task();
        let fut2 = fndmntls::use_ready_fut() ;
    }

    tokio::select! {
        _ = &mut fut1 =>{println!("fut 1")}
        _= &mut fut2 =>{println!("fut 2")}
    }

    //task_local macro makes number os static declaration local to the current task
    tokio::task_local! {
        pub static ONE: u32;

        #[allow(unused)]
        static TWO: f32;
    }
}
