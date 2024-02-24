use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{mpsc, Arc};
use std::task::{Context, Poll};
use std::{env, time::Duration};
use thirtyfour::prelude::*;
use thirtyfour::session::handle::SessionHandle;
use tokio::sync::Mutex;
static mut POOL: Option<WebDriverPool> = None;

type Job<A> = Box<
    dyn FnOnce(&SessionHandle) -> Box<dyn Future<Output = A> + 'static + Unpin> + 'static + Send,
>;

// New type implementing Future
struct JobFuture<A> {
    job: Option<Job<A>>,
    session_handle: SessionHandle,
}

impl<'a, A> Future for JobFuture<A> {
    type Output = A;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let job = self.job.take().expect("polled after completion");
        let future = job(&self.session_handle);
        // Pin the future and poll it
        let mut pinned_future = Box::pin(future);
        pinned_future.as_mut().poll(cx)
    }
}
struct WdPool<A> {
    workers: Vec<Worker>,
    senders: mpsc::Sender<Job<A>>,
}

impl<A> WdPool<A> {
    async fn new(max_workers: usize) -> WebDriverResult<WdPool<A>> {
        let mut workers = Vec::with_capacity(max_workers);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..max_workers {
            let driver = start_driver().await?;
            let worker = Worker {
                id: driver.session_id().await.unwrap().to_string(),
                session: driver.handle,
            };
            workers.push(worker);
        }
        Ok(WdPool {
            workers,
            senders: tx,
        })
    }

    fn execute<F>(&self, job: Job<A>)
    where
        A: 'static,
    {
        self.senders
            .send(Box::new(move |handle| job(handle)))
            .unwrap();
    }
}

struct Worker {
    id: String,
    session: SessionHandle,
}

impl Worker {
    async fn new<A>(
        id: String,
        receiver: Arc<Mutex<mpsc::Receiver<Job<A>>>>,
        session: SessionHandle,
    ) -> Worker
    where
        A: 'static + Send,
    {
        let session_handle = session.clone(); // Cloning the session handle for tokio::spawn
        tokio::spawn(async move {
            let job_future = JobFuture::<A> {
                job: Some(receiver.lock().await.recv().unwrap()),
                session_handle: session_handle, // Cloning inside the closure
            };
            job_future.await;
        });

        Worker { id, session }
    }
}

pub struct WebDriverPool {
    pub workers: Vec<WebDriver>,
}

impl WebDriverPool {
    pub async fn new(max_workers: usize) -> WebDriverResult<WebDriverPool> {
        let mut workers = Vec::with_capacity(max_workers);
        for _ in 0..max_workers {
            let driver = start_driver().await?;
            workers.push(driver);
        }
        Ok(WebDriverPool { workers })
    }

    pub async fn get_driver(&mut self) -> Option<WebDriver> {
        loop {
            if let Some(mut driver) = self.workers.pop() {
                if !is_alive(&driver).await {
                    driver = start_driver().await.unwrap();
                }
                return Some(driver);
            } else {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    pub fn return_driver(&mut self, driver: WebDriver) {
        self.workers.push(driver);
    }
}

async fn is_alive(driver: &WebDriver) -> bool {
    match driver.delete_cookie("").await {
        // just make some action with driver to get an error
        Ok(_) => true,
        Err(_) => false,
    }
}

/// This function get a pool that has instantiated with `init_global_pool`
/// If has not been initialized yet. The function will initialize the pool
///
/// # Returns
///  `WebDriverResult<&'static mut WebDriverPool>`
///
/// # Example
/// ``` rust
///     // Init pool or get it from `get_global_pool()`;
///     init_global_pool().await;
///     let pool = get_global_pool().await.expect("Cannot get global pool");
///
///     // Get a driver instance of pool to use
///     let driver = pool.get_driver().await.unwrap();
///
///     driver.goto("https://www.rust-lang.org").await.unwrap();
///     assert_eq!(driver.title().await.unwrap(), "Rust Programming Language");
///
///     //after use return driver for pool
///     pool.return_driver(driver);
/// ```
pub async fn get_global_pool() -> WebDriverResult<&'static mut WebDriverPool> {
    let pool: &mut Option<WebDriverPool> = unsafe { &mut POOL };
    if pool.is_none() {
        init_global_pool().await;
    }
    Ok(pool.as_mut().unwrap())
}

pub async fn init_global_pool() {
    // await for init selenium server
    tokio::time::sleep(tokio::time::Duration::new(4, 0)).await;

    let pool: &mut Option<WebDriverPool> = unsafe { &mut POOL };
    let max_nodes: usize = match env::var("POOL_SIZE") {
        Ok(n) => n
            .parse()
            .expect("cannot parse POOL_SIZE, must be a positive integer"),
        Err(_) => {
            println!("POOL_SIZE no declared, set to 1");
            1
        }
    };

    *pool = Some(WebDriverPool::new(max_nodes).await.unwrap());
}

///  This function assume that you have a web server of selenium running on port 4444
///  
///  # Returns
///  A `WebDriverResult` containing a `WebDriver` struct with extracted tags.
///  
///  # Example
/// ```rust
///   let driver = start_driver();
///   driver.goto("your_web_page.com");
/// ```
async fn start_driver() -> WebDriverResult<WebDriver> {
    // Configure Firefox options to run in headless mode.
    let mut caps = DesiredCapabilities::firefox();
    caps.add_firefox_arg("-headless")
        .expect("Cannot open Firefox without window");
    // Create a new WebDriver instance.
    let driver = WebDriver::new("http://0.0.0.0:4444/", caps).await?;
    Ok(driver)
}
