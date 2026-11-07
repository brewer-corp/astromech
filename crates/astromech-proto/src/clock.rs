//! The one clock the core reads time and waits through (strategy one-tested-process: "the core
//! reads time through a clock the tests control").
//!
//! The installed app runs on [`SystemClock`], whose every call is the `std` call the core made
//! before it had a clock: `Instant::now`, `SystemTime::now`, `thread::sleep`, `mpsc` and
//! `Condvar` waits with the same timeouts. A world (decision astromech-e1a5) runs on a
//! [`ManualClock`]: time moves only when the script calls [`ManualClock::advance`], which wakes
//! every clock wait whose deadline passed and returns once the core has finished what was due.
//!
//! The rules for code that waits on it are on [`Clock`].

use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, RecvError, RecvTimeoutError, SendError, TryRecvError};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The time, and waits that a test can move.
///
/// Implement [`now`](Clock::now), [`utc`](Clock::utc) and [`sleep`](Clock::sleep); the waits
/// that a clock can wake (channels, condvars, [`spawn`](trait.Clock.html#method.spawn)) are
/// methods on `dyn Clock`, real time unless the clock is a [`ManualClock`].
///
/// # Rules
///
/// - **A registered thread blocks only in clock waits**: [`Clock::sleep`], a
///   [`ClockReceiver`]'s `recv`/`recv_timeout`, and the condvar waits on `dyn Clock`
///   ([`wait`](trait.Clock.html#method.wait), [`wait_timeout`](trait.Clock.html#method.wait_timeout)).
///   `advance` waits for every registered thread to be parked in one; a thread blocked anywhere
///   else (a `std` channel, a `join` of a thread that needs clock time, a long sleep) holds it
///   up. Network and disk I/O, a lock held briefly and a `join` of a thread that was stopped and
///   woken (rule 3) are work, not waits: `advance` waits for them to finish.
/// - **A thread is registered by its parent before it is spawned**: [`Clock::register`] is taken
///   on the parent and moved into the child, which [enters](Registration::enter) it first, so the
///   child counts from its first instruction ([`spawn`](trait.Clock.html#method.spawn) does
///   both). A thread registered only once it runs could be missed by an `advance` in between.
/// - **Whatever stops a clock-waiting thread wakes its wait**: set the stop flag, then
///   [`ClockSender::wake`] its channel or [`notify_all`](trait.Clock.html#method.notify_all) its
///   condvar. Under the manual clock a timed wait ends only on its deadline, so a stop that
///   doesn't wake waits for an `advance`. A receiver never sees its senders disconnect under
///   the manual clock (it holds one for its wakes), so a loop that ends only when its senders
///   go needs its own stop and wake.
/// - **File watchers send into a clock channel** (`notify`'s handler calls
///   [`ClockSender::send`]), so an event marks its receiver busy before `advance` can see it
///   parked.
/// - **Untimed waits count as parked**: a registered thread in `recv()` or `wait()` is quiescent;
///   only data, a notify or a stop wakes it.
/// - Unregistered threads' timed clock waits also end only by `advance`, but `advance` doesn't
///   wait for those threads. Short pacing waits that finish on their own (a key hold, a file
///   retry) stay on real time, each with `#[allow(clippy::disallowed_methods)]` and its reason.
pub trait Clock: Send + Sync {
    /// Monotonic now, for timers and elapsed times.
    fn now(&self) -> Instant;
    /// UTC now, for stamps compared with the game's.
    fn utc(&self) -> SystemTime;
    /// UTC now in Unix seconds.
    fn now_s(&self) -> i64 {
        self.utc().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
    }
    /// Block for `d`.
    fn sleep(&self, d: Duration);
    /// A registration for a thread about to be spawned: move it into the thread and
    /// [enter](Registration::enter) it first. Real clocks count nothing, so theirs is empty.
    fn register(&self) -> Registration {
        Registration(None)
    }
    /// The manual clock behind this one, if any (the waits on `dyn Clock` use it).
    #[doc(hidden)]
    fn manual(&self) -> Option<&Arc<Manual>> {
        None
    }
}

/// The system's clocks: `Instant::now`, `SystemTime::now` and `thread::sleep`.
pub struct SystemClock;

#[allow(clippy::disallowed_methods, reason = "the system clock is the one place the core reads real time")]
impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
    fn utc(&self) -> SystemTime {
        SystemTime::now()
    }
    fn sleep(&self, d: Duration) {
        std::thread::sleep(d)
    }
}

impl dyn Clock {
    /// A channel whose receiver waits on this clock. On [`SystemClock`] it is a
    /// `std::sync::mpsc` channel.
    pub fn channel<T: Send>(&self) -> (ClockSender<T>, ClockReceiver<T>) {
        let (tx, rx) = mpsc::channel();
        let manual = self.manual().map(|m| (m.clone(), m.lock().fresh_id()));
        let waker = manual.as_ref().map(|_| tx.clone());
        (ClockSender { tx, manual: manual.clone() }, ClockReceiver { rx, manual, waker })
    }

    /// `cv.wait(guard)`, a wait the clock can see: `guard` is `m`'s.
    pub fn wait<'a, T: Send + 'static>(
        &self,
        m: &'a Arc<Mutex<T>>,
        cv: &Arc<Condvar>,
        guard: MutexGuard<'a, T>,
    ) -> MutexGuard<'a, T> {
        let Some(man) = self.manual() else {
            return cv.wait(guard).unwrap_or_else(|e| e.into_inner());
        };
        let id = man.begin(Deadline::Never, Key::Cv(cv_key(cv)), Some(cv_waker(m, cv))).expect("an untimed wait");
        let g = cv.wait(guard).unwrap_or_else(|e| e.into_inner());
        man.end(id);
        g
    }

    /// `cv.wait_timeout(guard, d)`, a wait the clock can see and end: `guard` is `m`'s. True
    /// when it timed out. Like `Condvar`'s, it can return early: check the condition again.
    pub fn wait_timeout<'a, T: Send + 'static>(
        &self,
        m: &'a Arc<Mutex<T>>,
        cv: &Arc<Condvar>,
        guard: MutexGuard<'a, T>,
        d: Duration,
    ) -> (MutexGuard<'a, T>, bool) {
        let Some(man) = self.manual() else {
            #[allow(clippy::disallowed_methods, reason = "the system clock's wait is std's")]
            let (g, r) = cv.wait_timeout(guard, d).unwrap_or_else(|e| e.into_inner());
            return (g, r.timed_out());
        };
        if d.is_zero() {
            return (guard, true);
        }
        man.cv_wait(m, cv, guard, Deadline::After(d))
    }

    /// [`wait_timeout`](trait.Clock.html#method.wait_timeout) for `d` measured from `now`, an
    /// earlier read of this clock: on [`SystemClock`] exactly `cv.wait_timeout(guard, d)`. Under
    /// the manual clock an `advance` between the caller's `now()` and this wait can't push the
    /// deadline back; one already past times out at once.
    pub fn wait_timeout_from<'a, T: Send + 'static>(
        &self,
        m: &'a Arc<Mutex<T>>,
        cv: &Arc<Condvar>,
        guard: MutexGuard<'a, T>,
        now: Instant,
        d: Duration,
    ) -> (MutexGuard<'a, T>, bool) {
        let Some(man) = self.manual() else {
            #[allow(clippy::disallowed_methods, reason = "the system clock's wait is std's")]
            let (g, r) = cv.wait_timeout(guard, d).unwrap_or_else(|e| e.into_inner());
            return (g, r.timed_out());
        };
        if d.is_zero() {
            return (guard, true);
        }
        man.cv_wait(m, cv, guard, Deadline::At(man.offset(now) + d))
    }

    /// [`Clock::sleep`] for `d` measured from `now`, an earlier read of this clock: on
    /// [`SystemClock`] exactly `thread::sleep(d)`. Under the manual clock it ends at `now + d`
    /// even if an `advance` came between the caller's `now()` and this call.
    pub fn sleep_from(&self, now: Instant, d: Duration) {
        match self.manual() {
            None => self.sleep(d),
            Some(man) if !d.is_zero() => man.sleep_until(man.offset(now) + d),
            Some(_) => {}
        }
    }

    /// `cv.notify_all()`. Under the manual clock it first marks the threads waiting on `cv` as
    /// busy and takes `m` once, so a waiter between its check and its wait isn't missed. Don't
    /// hold `m` when calling it.
    pub fn notify_all<T>(&self, m: &Mutex<T>, cv: &Condvar) {
        if let Some(man) = self.manual() {
            man.lock().busy(Key::Cv(cv_key(cv)));
            drop(m.lock());
        }
        cv.notify_all();
    }

    /// Under the manual clock, block until every registered thread but the caller is parked in
    /// a clock wait or gone, or `guard` (real time) has passed: false then. True at once on a
    /// real clock. For a registered thread a world holds still while the rest settles.
    #[allow(clippy::disallowed_methods, reason = "a real-time hang guard: it only ends a wait that never settles")]
    pub fn wait_others_parked(&self, guard: Duration) -> bool {
        let Some(man) = self.manual() else { return true };
        let me = CURRENT.with(|c| c.get()).filter(|(clock, _)| *clock == man.id).map(|(_, r)| r);
        let deadline = Instant::now() + guard;
        let mut s = man.lock();
        loop {
            if !s.regs.iter().any(|(r, parked)| !parked && Some(*r) != me) {
                return true;
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return false;
            }
            s = man.settled.wait_timeout(s, left).unwrap_or_else(|e| e.into_inner()).0;
        }
    }

    /// Spawn `f` on a thread registered with this clock: the registration is taken here, on
    /// the parent, and entered first thing on the child.
    pub fn spawn<F, R>(&self, builder: std::thread::Builder, f: F) -> std::io::Result<std::thread::JoinHandle<R>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let reg = self.register();
        builder.spawn(move || {
            let _entered = reg.enter();
            f()
        })
    }
}

fn cv_key(cv: &Condvar) -> usize {
    cv as *const Condvar as usize
}

/// How the manual clock ends a condvar wait: take the lock (so the waiter is inside `wait`),
/// then notify.
fn cv_waker<T: Send + 'static>(m: &Arc<Mutex<T>>, cv: &Arc<Condvar>) -> Waker {
    let (m, cv) = (m.clone(), cv.clone());
    Arc::new(move || {
        drop(m.lock());
        cv.notify_all();
    })
}

enum Msg<T> {
    Item(T),
    /// End the wait with this id ([`STOP`]: whichever wait is running, or the next).
    Wake(u64),
}

/// The wake a stop sends ([`ClockSender::wake`]).
const STOP: u64 = 0;

/// The sending half of [`channel`](trait.Clock.html#method.channel).
pub struct ClockSender<T> {
    tx: mpsc::Sender<Msg<T>>,
    manual: Option<(Arc<Manual>, u64)>,
}

impl<T> Clone for ClockSender<T> {
    fn clone(&self) -> Self {
        Self { tx: self.tx.clone(), manual: self.manual.clone() }
    }
}

impl<T> ClockSender<T> {
    /// `Sender::send`.
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        let r = match &self.manual {
            None => self.tx.send(Msg::Item(t)),
            Some((m, chan)) => {
                // Under the clock's lock: the receiver either takes it before it parks or is
                // marked busy here.
                let mut s = m.lock();
                s.busy(Key::Chan(*chan));
                self.tx.send(Msg::Item(t))
            }
        };
        r.map_err(|SendError(m)| match m {
            Msg::Item(t) => SendError(t),
            Msg::Wake(_) => unreachable!("sent an item"),
        })
    }

    /// End the receiver's wait now (its `recv_timeout` times out, its `recv` fails): for a
    /// stop, after setting the stop flag the receiving loop checks.
    pub fn wake(&self) {
        match &self.manual {
            None => {
                let _ = self.tx.send(Msg::Wake(STOP));
            }
            Some((m, chan)) => {
                let mut s = m.lock();
                s.busy(Key::Chan(*chan));
                let _ = self.tx.send(Msg::Wake(STOP));
            }
        }
    }
}

/// The receiving half of [`channel`](trait.Clock.html#method.channel).
pub struct ClockReceiver<T> {
    rx: mpsc::Receiver<Msg<T>>,
    manual: Option<(Arc<Manual>, u64)>,
    /// The manual clock's way in for a deadline (so its senders never all go).
    waker: Option<mpsc::Sender<Msg<T>>>,
}

impl<T: Send + 'static> ClockReceiver<T> {
    /// `Receiver::recv_timeout(d)`. A [wake](ClockSender::wake) is a timeout.
    pub fn recv_timeout(&self, d: Duration) -> Result<T, RecvTimeoutError> {
        let (Some((m, chan)), Some(w)) = (&self.manual, &self.waker) else {
            #[allow(clippy::disallowed_methods, reason = "the system clock's wait is std's")]
            return match self.rx.recv_timeout(d) {
                Ok(Msg::Item(t)) => Ok(t),
                Ok(Msg::Wake(_)) => Err(RecvTimeoutError::Timeout),
                Err(e) => Err(e),
            };
        };
        let w = w.clone();
        let wake: Waker1 = Arc::new(move |id| {
            let _ = w.send(Msg::Wake(id));
        });
        let id = match self.park(m, *chan, if d.is_zero() { None } else { Some((Deadline::After(d), wake)) }) {
            Parked::Got(t) => return Ok(t),
            Parked::Stop | Parked::Empty => return Err(RecvTimeoutError::Timeout),
            Parked::Waiting(id) => id,
        };
        let r = self.until(id);
        m.end(id);
        r.map_err(|()| RecvTimeoutError::Timeout)
    }

    /// `Receiver::recv()`. A [wake](ClockSender::wake) is an error, as when the senders are
    /// gone.
    pub fn recv(&self) -> Result<T, RecvError> {
        let Some((m, chan)) = &self.manual else {
            return match self.rx.recv() {
                Ok(Msg::Item(t)) => Ok(t),
                Ok(Msg::Wake(_)) | Err(_) => Err(RecvError),
            };
        };
        let noop: Waker1 = Arc::new(|_| {});
        let id = match self.park(m, *chan, Some((Deadline::Never, noop))) {
            Parked::Got(t) => return Ok(t),
            Parked::Stop | Parked::Empty => return Err(RecvError),
            Parked::Waiting(id) => id,
        };
        let r = self.until(id);
        m.end(id);
        r.map_err(|()| RecvError)
    }

    /// `Receiver::try_recv()`, skipping wakes.
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        loop {
            match self.rx.try_recv()? {
                Msg::Item(t) => return Ok(t),
                Msg::Wake(_) => continue,
            }
        }
    }

    /// Under the clock's lock, take what is queued or start a wait.
    fn park(&self, m: &Arc<Manual>, chan: u64, wait: Option<(Deadline, Waker1)>) -> Parked<T> {
        let mut s = m.lock();
        loop {
            match self.rx.try_recv() {
                Ok(Msg::Item(t)) => return Parked::Got(t),
                Ok(Msg::Wake(STOP)) => return Parked::Stop,
                Ok(Msg::Wake(_)) => continue,
                Err(_) => break,
            }
        }
        match wait {
            None => Parked::Empty,
            Some((deadline, wake)) => {
                let id = s.fresh_id();
                let waker: Waker = Arc::new(move || wake(id));
                Parked::Waiting(s.begin(m, id, deadline, Key::Chan(chan), Some(waker)))
            }
        }
    }

    /// Block until an item, a stop or this wait's own wake. Err: woken.
    fn until(&self, id: u64) -> Result<T, ()> {
        loop {
            match self.rx.recv() {
                Ok(Msg::Item(t)) => return Ok(t),
                Ok(Msg::Wake(w)) if w == STOP || w == id => return Err(()),
                Ok(Msg::Wake(_)) => continue,
                // Unreachable while `waker` holds a sender; treated as woken.
                Err(_) => return Err(()),
            }
        }
    }
}

type Waker = Arc<dyn Fn() + Send + Sync>;
type Waker1 = Arc<dyn Fn(u64) + Send + Sync>;

enum Parked<T> {
    Got(T),
    Stop,
    Empty,
    Waiting(u64),
}

/// Held by a registered thread ([`Clock::register`]). Dropping it unregisters the thread.
#[must_use = "move the registration into the thread and enter it first"]
pub struct Registration(Option<(Arc<Manual>, u64)>);

impl Registration {
    /// Make this thread the registered one: call it first thing on the child.
    pub fn enter(self) -> Entered {
        if let Some((m, id)) = &self.0 {
            CURRENT.with(|c| c.set(Some((m.id, *id))));
        }
        Entered { reg: self, _here: PhantomData }
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        if let Some((m, id)) = self.0.take() {
            let mut s = m.lock();
            s.regs.remove(&id);
            m.settled.notify_all();
        }
    }
}

/// A [`Registration`] entered on this thread; dropping it unregisters the thread.
pub struct Entered {
    reg: Registration,
    _here: PhantomData<*const ()>,
}

impl Drop for Entered {
    fn drop(&mut self) {
        if self.reg.0.is_some() {
            CURRENT.with(|c| c.set(None));
        }
    }
}

thread_local! {
    /// The manual clock and registration this thread entered.
    static CURRENT: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
}

static CLOCKS: AtomicU64 = AtomicU64::new(1);

/// A clock that moves only when [`advance`](ManualClock::advance) moves it.
pub struct ManualClock(Arc<Manual>);

/// The manual clock's state, shared with its channels and registrations.
#[doc(hidden)]
pub struct Manual {
    id: u64,
    mono: Instant,
    utc: SystemTime,
    state: Mutex<State>,
    /// Told each time a registered thread parks or goes.
    settled: Condvar,
    /// Told how many registered threads are still busy each time `advance` waits for them.
    #[cfg(test)]
    waiting: Mutex<Option<mpsc::Sender<usize>>>,
    /// Run once by the next sleep, just before it registers its wait.
    #[cfg(test)]
    before_sleep: Mutex<Option<Box<dyn FnOnce() + Send>>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Key {
    Chan(u64),
    Cv(usize),
    Sleep,
}

enum Deadline {
    Never,
    After(Duration),
    At(Duration),
}

struct Wait {
    reg: Option<u64>,
    /// Since the clock's start.
    deadline: Option<Duration>,
    key: Key,
    waker: Option<Waker>,
    fired: bool,
}

#[derive(Default)]
struct State {
    /// Time since the clock's start.
    elapsed: Duration,
    next: u64,
    /// Registered threads, and whether each is parked in a clock wait.
    regs: HashMap<u64, bool>,
    waits: HashMap<u64, Wait>,
}

impl State {
    fn fresh_id(&mut self) -> u64 {
        self.next += 1;
        self.next
    }

    /// The registered threads waiting on `key` are busy: something woke them.
    fn busy(&mut self, key: Key) {
        let regs: Vec<u64> = self.waits.values().filter(|w| w.key == key).filter_map(|w| w.reg).collect();
        for r in regs {
            if let Some(p) = self.regs.get_mut(&r) {
                *p = false;
            }
        }
    }

    fn begin(&mut self, m: &Manual, id: u64, deadline: Deadline, key: Key, waker: Option<Waker>) -> u64 {
        let reg = CURRENT.with(|c| c.get()).filter(|(clock, r)| *clock == m.id && self.regs.contains_key(r));
        let reg = reg.map(|(_, r)| r);
        let deadline = match deadline {
            Deadline::Never => None,
            Deadline::After(d) => Some(self.elapsed + d),
            Deadline::At(t) => Some(t),
        };
        self.waits.insert(id, Wait { reg, deadline, key, waker, fired: false });
        if let Some(r) = reg {
            self.regs.insert(r, true);
            m.settled.notify_all();
        }
        id
    }
}

impl Manual {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Start a wait, checking its deadline under the same lock: None when it has passed
    /// already (an `advance` between the caller's reading and here can't be lost).
    fn begin(&self, deadline: Deadline, key: Key, waker: Option<Waker>) -> Option<u64> {
        let mut s = self.lock();
        if let Deadline::At(t) = deadline {
            if t <= s.elapsed {
                return None;
            }
        }
        let id = s.fresh_id();
        Some(s.begin(self, id, deadline, key, waker))
    }

    /// `t` as time since the clock's start.
    fn offset(&self, t: Instant) -> Duration {
        t.saturating_duration_since(self.mono)
    }

    /// A condvar wait to `deadline`; true if the deadline ended it.
    fn cv_wait<'a, T: Send + 'static>(
        &self,
        m: &Arc<Mutex<T>>,
        cv: &Arc<Condvar>,
        guard: MutexGuard<'a, T>,
        deadline: Deadline,
    ) -> (MutexGuard<'a, T>, bool) {
        let Some(id) = self.begin(deadline, Key::Cv(cv_key(cv)), Some(cv_waker(m, cv))) else {
            return (guard, true);
        };
        let g = cv.wait(guard).unwrap_or_else(|e| e.into_inner());
        (g, self.end(id))
    }

    /// The wait is over; true if its deadline ended it.
    fn end(&self, id: u64) -> bool {
        let mut s = self.lock();
        let w = s.waits.remove(&id);
        if let Some(r) = w.as_ref().and_then(|w| w.reg) {
            if let Some(p) = s.regs.get_mut(&r) {
                *p = false;
            }
        }
        w.is_some_and(|w| w.fired)
    }

    fn sleep(&self, d: Duration) {
        if d.is_zero() {
            return;
        }
        let until = self.lock().elapsed + d;
        self.sleep_until(until);
    }

    /// Sleep until `until` (time since the clock's start).
    fn sleep_until(&self, until: Duration) {
        let (m, cv) = (Arc::new(Mutex::new(())), Arc::new(Condvar::new()));
        let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            #[cfg(test)]
            if let Some(f) = self.before_sleep.lock().unwrap_or_else(|e| e.into_inner()).take() {
                f();
            }
            // The deadline is checked and the wait registered under one lock.
            let Some(id) = self.begin(Deadline::At(until), Key::Sleep, Some(cv_waker(&m, &cv))) else {
                return;
            };
            g = cv.wait(g).unwrap_or_else(|e| e.into_inner());
            if self.end(id) {
                return;
            }
        }
    }
}

impl ManualClock {
    /// A clock whose UTC now is `utc` until it is advanced.
    pub fn new(utc: SystemTime) -> Self {
        ManualClock(Arc::new(Manual {
            id: CLOCKS.fetch_add(1, Ordering::Relaxed),
            #[allow(
                clippy::disallowed_methods,
                reason = "the manual clock's origin: Instant has no other constructor"
            )]
            mono: Instant::now(),
            utc,
            state: Mutex::default(),
            settled: Condvar::new(),
            #[cfg(test)]
            waiting: Mutex::new(None),
            #[cfg(test)]
            before_sleep: Mutex::new(None),
        }))
    }

    /// [`ManualClock::new`] at `secs` Unix seconds.
    pub fn at_unix(secs: i64) -> Self {
        Self::new(UNIX_EPOCH + Duration::from_secs(secs.max(0) as u64))
    }

    /// Move time on by `d`, wake every clock wait whose deadline has passed, then return once
    /// every registered thread is parked in a clock wait again (or gone). `advance(ZERO)` only
    /// waits for that. Call it from an unregistered thread: a registered caller would wait for
    /// itself.
    pub fn advance(&self, d: Duration) {
        let m = &self.0;
        let wakers: Vec<Waker> = {
            let mut s = m.lock();
            s.elapsed += d;
            let now = s.elapsed;
            let due: Vec<(Key, Option<u64>, Option<Waker>)> = s
                .waits
                .values_mut()
                .filter(|w| !w.fired && w.deadline.is_some_and(|t| t <= now))
                .map(|w| {
                    w.fired = true;
                    (w.key, w.reg, w.waker.clone())
                })
                .collect();
            let mut wakers = vec![];
            for (key, reg, waker) in due {
                if let Some(p) = reg.and_then(|r| s.regs.get_mut(&r)) {
                    *p = false;
                }
                // A condvar's notify wakes every waiter on it.
                if matches!(key, Key::Cv(_)) {
                    s.busy(key);
                }
                wakers.extend(waker);
            }
            wakers
        };
        for w in wakers {
            w();
        }
        let mut s = m.lock();
        loop {
            let busy = s.regs.values().filter(|parked| !**parked).count();
            if busy == 0 {
                return;
            }
            #[cfg(test)]
            if let Some(tx) = m.waiting.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
                let _ = tx.send(busy);
            }
            s = m.settled.wait(s).unwrap_or_else(|e| e.into_inner());
        }
    }
}

impl Clock for ManualClock {
    fn now(&self) -> Instant {
        self.0.mono + self.0.lock().elapsed
    }
    fn utc(&self) -> SystemTime {
        self.0.utc + self.0.lock().elapsed
    }
    fn sleep(&self, d: Duration) {
        self.0.sleep(d)
    }
    fn register(&self) -> Registration {
        let mut s = self.0.lock();
        let id = s.fresh_id();
        s.regs.insert(id, false);
        Registration(Some((self.0.clone(), id)))
    }
    fn manual(&self) -> Option<&Arc<Manual>> {
        Some(&self.0)
    }
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // tests are exempt for now (astromech-c7lc)
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    const S: Duration = Duration::from_secs(1);

    fn manual() -> (Arc<ManualClock>, Arc<dyn Clock>) {
        let m = Arc::new(ManualClock::at_unix(1_700_000_000));
        let c: Arc<dyn Clock> = m.clone();
        (m, c)
    }

    /// Told how many registered threads are busy each time `advance` waits for them.
    fn on_waiting(m: &ManualClock) -> mpsc::Receiver<usize> {
        let (tx, rx) = channel();
        *m.0.waiting.lock().unwrap() = Some(tx);
        rx
    }

    #[derive(Debug, PartialEq)]
    enum Seen {
        Woken,
        /// `advance` is waiting for this many registered threads.
        AdvanceWaiting(usize),
        AdvanceReturned,
    }

    /// `advance` on its own thread, reporting into `seen` when it waits and when it returns.
    fn advance_on_a_thread(
        m: &Arc<ManualClock>,
        d: Duration,
        seen: &mpsc::Sender<Seen>,
    ) -> std::thread::JoinHandle<()> {
        let waiting = on_waiting(m);
        let (m, s, s2) = (m.clone(), seen.clone(), seen.clone());
        std::thread::spawn(move || {
            for n in waiting {
                let _ = s2.send(Seen::AdvanceWaiting(n));
            }
        });
        std::thread::spawn(move || {
            m.advance(d);
            *m.0.waiting.lock().unwrap() = None;
            let _ = s.send(Seen::AdvanceReturned);
        })
    }

    /// A registered thread waits for the others: it returns once the busy one parks, and not
    /// before (the busy one is held at a gate the test opens).
    #[test]
    fn a_thread_waits_until_the_others_are_parked() {
        let (m, c) = manual();
        let (gate_tx, gate) = channel::<()>();
        let c2 = c.clone();
        let busy = c
            .spawn(std::thread::Builder::new(), move || {
                let _ = gate.recv();
                c2.sleep(S);
            })
            .unwrap();
        let (done_tx, done) = channel();
        let c3 = c.clone();
        let waiter = c
            .spawn(std::thread::Builder::new(), move || {
                done_tx.send(c3.wait_others_parked(Duration::from_secs(600))).unwrap();
            })
            .unwrap();
        assert!(!c.wait_others_parked(Duration::ZERO), "an unregistered caller counts everyone");
        assert!(done.try_recv().is_err(), "returned while the other was busy");
        gate_tx.send(()).unwrap();
        assert!(done.recv().unwrap());
        waiter.join().unwrap();
        m.advance(S);
        busy.join().unwrap();
        let system: Arc<dyn Clock> = Arc::new(SystemClock);
        assert!(system.wait_others_parked(Duration::ZERO));
    }

    /// The race: a registered thread loops on `sleep(1s)`, says it woke, then waits on a gate
    /// the test holds. `advance(1s)` must not return while the gate is closed: the test sees
    /// `advance` waiting for the thread before it opens the gate, and `advance` returning only
    /// after. A clock whose `advance` returns once the sleeper is merely woken reports
    /// `AdvanceReturned` with the gate still closed.
    #[test]
    fn advance_returns_only_once_the_woken_thread_is_parked_again() {
        let (m, c) = manual();
        let (seen_tx, seen) = channel();
        let (gate_tx, gate) = channel::<()>();
        let t = seen_tx.clone();
        let c2 = c.clone();
        let sleeper = c
            .spawn(std::thread::Builder::new(), move || {
                for _ in 0..2 {
                    c2.sleep(S);
                    let _ = t.send(Seen::Woken);
                    if gate.recv().is_err() {
                        return;
                    }
                }
            })
            .unwrap();
        // Parked in its first sleep.
        m.advance(Duration::ZERO);
        let adv = advance_on_a_thread(&m, S, &seen_tx);
        let mut got = vec![];
        // Until both the wake and advance's wait are seen, with the gate closed.
        while !(got.contains(&Seen::Woken) && got.iter().any(|s| matches!(s, Seen::AdvanceWaiting(_)))) {
            let s = seen.recv().unwrap();
            assert_ne!(
                s,
                Seen::AdvanceReturned,
                "advance returned while the woken thread was held at the gate: {got:?}"
            );
            got.push(s);
        }
        assert!(got.contains(&Seen::AdvanceWaiting(1)), "{got:?}");
        gate_tx.send(()).unwrap();
        loop {
            match seen.recv().unwrap() {
                Seen::AdvanceReturned => break,
                Seen::AdvanceWaiting(_) => {}
                Seen::Woken => panic!("woken twice by one advance"),
            }
        }
        adv.join().unwrap();
        drop(gate_tx);
        m.advance(S);
        sleeper.join().unwrap();
    }

    /// A child spawned with its parent's registration counts from its first instruction: an
    /// `advance` while the child is held before it ever waits on the clock waits for it.
    #[test]
    fn a_child_counts_from_its_parents_registration() {
        let (m, c) = manual();
        let (seen_tx, seen) = channel();
        let (gate_tx, gate) = channel::<()>();
        let (started_tx, started) = channel();
        let reg = c.register();
        let c2 = c.clone();
        let child = std::thread::spawn(move || {
            let _ = started_tx.send(());
            // Held before entering the registration, and before any clock wait.
            gate.recv().unwrap();
            let _in = reg.enter();
            c2.sleep(S);
        });
        started.recv().unwrap();
        let adv = advance_on_a_thread(&m, Duration::ZERO, &seen_tx);
        assert_eq!(seen.recv().unwrap(), Seen::AdvanceWaiting(1), "advance didn't wait for the child");
        gate_tx.send(()).unwrap();
        loop {
            match seen.recv().unwrap() {
                Seen::AdvanceReturned => break,
                Seen::AdvanceWaiting(n) => assert_eq!(n, 1),
                Seen::Woken => unreachable!(),
            }
        }
        adv.join().unwrap();
        m.advance(S);
        child.join().unwrap();
    }

    /// A channel wait times out on `advance` past its deadline, not before; an item ends it at
    /// once; a wake ends a timed or an untimed wait.
    #[test]
    fn channel_waits_end_by_deadline_item_or_wake() {
        let (m, c) = manual();
        let (tx, rx) = c.channel::<u32>();
        let (out_tx, out) = channel();
        let c2 = c.clone();
        let t = c
            .spawn(std::thread::Builder::new(), move || {
                let _ = out_tx.send(format!("{:?}", rx.recv_timeout(S * 2)));
                let _ = out_tx.send(format!("{:?}", rx.recv_timeout(S * 2)));
                let _ = out_tx.send(format!("{:?}", rx.recv_timeout(S * 2)));
                let _ = out_tx.send(format!("{:?}", rx.recv()));
                drop(c2);
            })
            .unwrap();
        m.advance(Duration::ZERO);
        m.advance(S);
        assert!(out.try_recv().is_err(), "timed out before its deadline");
        m.advance(S);
        assert_eq!(out.try_recv().unwrap(), "Err(Timeout)");
        tx.send(7).unwrap();
        m.advance(Duration::ZERO);
        assert_eq!(out.try_recv().unwrap(), "Ok(7)");
        tx.wake();
        m.advance(Duration::ZERO);
        assert_eq!(out.try_recv().unwrap(), "Err(Timeout)");
        tx.wake();
        t.join().unwrap();
        assert_eq!(out.try_recv().unwrap(), "Err(RecvError)");
    }

    /// A condvar wait times out by `advance`; a notify through the clock ends it at once.
    #[test]
    fn condvar_waits_end_by_deadline_or_notify() {
        let (m, c) = manual();
        let (lock, cv) = (Arc::new(Mutex::new(0u32)), Arc::new(Condvar::new()));
        let (out_tx, out) = channel();
        let (l2, cv2, c2) = (lock.clone(), cv.clone(), c.clone());
        let t = c
            .spawn(std::thread::Builder::new(), move || {
                let g = l2.lock().unwrap();
                let (g, timed_out) = c2.wait_timeout(&l2, &cv2, g, S);
                let _ = out_tx.send(timed_out);
                let mut g = g;
                while *g == 0 {
                    g = c2.wait(&l2, &cv2, g);
                }
                let _ = out_tx.send(true);
            })
            .unwrap();
        m.advance(Duration::ZERO);
        assert!(out.try_recv().is_err());
        m.advance(S);
        assert!(out.try_recv().unwrap(), "the deadline ends the wait");
        *lock.lock().unwrap() = 1;
        c.notify_all(&lock, &cv);
        t.join().unwrap();
        assert!(out.try_recv().unwrap());
    }

    /// The system clock's channel is `mpsc`'s: a timeout after the real time, disconnection
    /// seen, and a wake is a timeout.
    #[test]
    fn system_channel_is_mpsc() {
        let c: Arc<dyn Clock> = Arc::new(SystemClock);
        let (tx, rx) = c.channel::<u32>();
        assert_eq!(rx.recv_timeout(Duration::from_millis(1)), Err(RecvTimeoutError::Timeout));
        tx.send(3).unwrap();
        assert_eq!(rx.recv_timeout(S), Ok(3));
        tx.wake();
        assert_eq!(rx.recv_timeout(S), Err(RecvTimeoutError::Timeout));
        drop(tx);
        assert_eq!(rx.recv(), Err(RecvError));
    }

    /// astromech-p78l: an `advance` that lands after a sleep has read its deadline but before
    /// it registers the wait still ends it. The hook runs the `advance` in exactly that gap;
    /// with the deadline checked under a separate lock from the wait's insert, the wake is
    /// lost and the sleep waits for another `advance`.
    #[test]
    fn an_advance_just_before_a_sleep_registers_still_ends_it() {
        let (m, c) = manual();
        let m2 = m.clone();
        *m.0.before_sleep.lock().unwrap() = Some(Box::new(move || m2.advance(S)));
        let (tx, rx) = channel();
        let c2 = c.clone();
        std::thread::spawn(move || {
            c2.sleep(S);
            let _ = tx.send(());
        });
        // A guard against hanging, never how the test decides: the sleep should end at once.
        assert!(rx.recv_timeout(Duration::from_secs(30)).is_ok(), "the advance before the sleep's wait was lost");
    }

    /// Waits measured from an earlier `now()` keep their deadline across an `advance` made in
    /// between: past it already, they end at once.
    #[test]
    fn waits_from_an_earlier_now_keep_their_deadline() {
        let (m, c) = manual();
        let now = c.now();
        m.advance(S);
        c.sleep_from(now, S);
        let (lock, cv) = (Arc::new(Mutex::new(())), Arc::new(Condvar::new()));
        let g = lock.lock().unwrap();
        let (_g, timed_out) = c.wait_timeout_from(&lock, &cv, g, now, S);
        assert!(timed_out);
    }

    #[test]
    fn manual_time_moves_only_by_advance() {
        let (m, c) = manual();
        let (t0, u0) = (c.now(), c.now_s());
        assert_eq!(u0, 1_700_000_000);
        m.advance(Duration::from_secs(90));
        assert_eq!(c.now() - t0, Duration::from_secs(90));
        assert_eq!(c.now_s(), u0 + 90);
    }
}
