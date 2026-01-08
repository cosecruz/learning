# Mastering Async

## 1. Async Foundations

- ### Futures as lazy state machines

- ### Poll trait & waker registration

- ### Pin: preventing self-referential moves

- ### Executor/reactor split

- ### Task scheduling & work stealing

- ### Context & waker propagation

- ### Async functions as generator syntax sugar

- ### Stream trait for async iteration

- ### AsyncRead/AsyncWrite traits

- ### Tokio Internals

---

## 2. Multi-threaded runtime architecture

- ### I/O driver (epoll/kqueue/IOCP)

- ### Timer wheel implementation

- ### Task spawning & local sets

- ### Runtime metrics & instrumentation

- ### Blocking operations: spawn_blocking

- ### Cooperative scheduling & yield points

- ### Cancellation: drop = cancel

---

## 3. Async Patterns

- ### select! for racing futures

- ### join! for parallel work

- ### timeout & retry patterns

- ### Async mutexes vs sync mutexes

- ### RwLock considerations in async

- ### Channels: mpsc, broadcast, watch, oneshot

- ### Semaphores for resource limiting

- ### Async trait workarounds (async-trait, RPITIT)

---

## 4. Backpressure & Flow Control

- ### Bounded channels as natural backpressure

- ### Buffering strategies

- ### Load shedding when overloaded

- ### Admission control

- ### Feedback loops
