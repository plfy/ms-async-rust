use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

struct Ready42;

impl Future for Ready42 {
    type Output = i32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready(42)
    }
}

struct Delay {
    completed: Arc<Mutex<bool>>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
    duration: Duration,
    started: bool,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Delay {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            duration,
            started: false,
        }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        // Check if already completed before storing waker
        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        // Store the waker - executor may pass a new one on each poll
        *self.waker_stored.lock().unwrap() = Some(cx.waker().clone());

        // Start the background timer on first poll
        if !self.started {
            self.started = true;
            let completed = Arc::clone(&self.completed);
            let waker = Arc::clone(&self.waker_stored);
            let duration = self.duration;

            thread::spawn(move || {
                thread::sleep(duration);
                *completed.lock().unwrap() = true;

                // CRITICAL: wake the executor so it polls us again
                if let Some(w) = waker.lock().unwrap().take() {
                    w.wake(); // "Hey executor, I'm ready — poll me again!"
                }
            });
        }

        // Double-check completion after storing waker (handles race condition)
        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        Poll::Pending // Not done yet
    }
}

#[tokio::test]
async fn test_delay() {
    let start = std::time::Instant::now();
    let delay = Delay::new(Duration::from_millis(2000));
    delay.await;
    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_millis(2000),
        "Delay should wait at least 100ms, but only waited {:?}",
        elapsed
    );
}

pub struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    pub fn new(start: u32) -> Self {
        Self { count: start }
    }
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("Liftoff!");
            Poll::Ready("Liftoff!")
        } else {
            println!("{}...", self.count);
            self.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::test]
async fn test_countdown() {
    let countdown = CountdownFuture::new(3);
    let result = countdown.await;
    assert_eq!(result, "Liftoff!")
}
