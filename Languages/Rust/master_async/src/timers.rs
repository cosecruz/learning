//timers

// Key idea:

// Time in Tokio is just another async resource
// (like sockets, channels, or tasks)

// Timers:

// are futures

// get polled

// register a waker

// wake when time advances

// These are the entire timer system:

// Item	What it represents
// Duration	Length of time (std)
// Instant	A point in time (Tokio’s clock)
// Sleep	A future that completes at an instant
// Interval	Repeating timer
// timeout	Wraps another future with a deadline

//examples

use std::result;
use std::time::Duration;

use tokio::time::{self, Instant, interval, sleep, sleep_until, timeout_at};

//1. job eith timeout
async fn do_t_job() {
    //simulate work
    time::sleep(Duration::from_millis(30)).await;
}

pub async fn timed_job() {
    let result = time::timeout(Duration::from_millis(20), do_t_job()).await;

    match result {
        Ok(_) => println!("job finished in time"),
        Err(_) => println!("job timed out"),
    }
}

//2. sleep
async fn example_sleep() {
    sleep(Duration::from_secs(1)).await;
    println!("one second passed")
}

//3. sleep_until
async fn example_sleep_untin() {
    let t = Instant::now() + Duration::from_secs(1);
    sleep_until(t).await;
    println!("reached instant");
}

//4: Interval
//for cron, heartbeat, polling loops
//use interval not sleep for stable executions; sleep usually drifts
pub async fn example_interval() {
    let mut ticker = interval(Duration::from_millis(500));

    for i in 0..5 {
        ticker.tick().await;
        println!("tick {}", i);
    }
}

//5. timeout_at - absolute deadline
async fn do_t_at_job(n: usize) -> Result<(), String> {
    if n == 0 {
        return Err(String::from("n is 0"));
    };
    //simulate job
    sleep(Duration::from_millis(40)).await;
    Ok(())
}
pub async fn example_timout_at() {
    let deadline = Instant::now() + Duration::from_secs(1);

    match timeout_at(deadline, do_t_at_job(1)).await {
        Err(_) => {
            // timeout
            println!("timed out");
        }
        Ok(Err(e)) => {
            // job ran, but failed
            println!("job failed: {}", e);
        }
        Ok(Ok(())) => {
            // job succeeded
            println!("job completed successfully");
        }
    }
}

//6. manual sleep advance but useful
async fn meanual_sleep() {
    let mut sleep = Box::pin(tokio::time::sleep(Duration::from_secs(1)));

    // can poll, reset, etc.
    sleep.as_mut().await;
    // sleep
    //     .as_mut()
    //     .reset(Instant::now() + Duration::from_secs(2));
}

async fn job(id: u32) {
    println!("job {} started", id);
    time::sleep(Duration::from_millis(30)).await;
    println!("job {} finished", id);
}

pub async fn timed_jobs() {
    let jobs = vec![1, 2, 3, 4];

    let mut interval = time::interval(Duration::from_millis(50));

    for id in jobs {
        interval.tick().await;

        let deadline = Instant::now() + Duration::from_millis(20);

        let result = time::timeout_at(deadline, job(id)).await;

        if result.is_err() {
            println!("job {} timed out", id);
        }
    }
}
