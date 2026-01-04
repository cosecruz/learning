use std::time::Duration;

use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|t| t.inner_html())
}

async fn run() {
    let arg1 = std::env::args().nth(1);
    let url = arg1.as_deref().unwrap_or("https://www.rust-lang.org");

    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    };
}

// async fn run_race() {
//     let args: Vec<String> = std::env::args().collect();

//     trpl::run(async {
//         let title_fut_1 = page_title(&args[1]);
//         let title_fut_2 = page_title(&args[2]);

//         let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
//             trpl::Either::Left(left) => left,
//             trpl::Either::Right(right) => right,
//         };

//         println!("{url} returned first");
//         match maybe_title {
//             Some(title) => println!("Its page title was: '{title}'"),
//             None => println!("It had no title."),
//         }
//     })
// }

async fn run_thread_async() {
    // let handle = trpl::run(async {
    //     trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // });

    // let handle = trpl::spawn_task(async {
    //     for i in 1..10 {
    //         println!("hi number {i} from the first task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the second task!");
    //     trpl::sleep(Duration::from_millis(500)).await;
    // }

    // handle.await.unwrap();

    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

fn main() {
    // trpl::run(async { run().await })
}
