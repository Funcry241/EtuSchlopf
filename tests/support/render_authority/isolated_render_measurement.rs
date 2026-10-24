// Each Rust test executes a separate project scenario. Preserve the production
// process-wide measurement implementation; isolate only cfg(test) storage so
// resets, pending-chain lifetimes and final-stage records cannot cross tests.
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;
use std::sync::atomic::Ordering;

thread_local! {
    static COUNTERS: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());
    static RECORDS: RefCell<HashMap<usize, Box<dyn Any>>> = RefCell::new(HashMap::new());
}

pub(super) struct AtomicUsize { initial: usize }
impl AtomicUsize {
    pub(super) const fn new(initial: usize) -> Self { Self { initial } }
    fn update(&self, operation: impl FnOnce(&mut usize)) -> usize {
        COUNTERS.with(|values| {
            let mut values = values.borrow_mut();
            let value = values.entry(std::ptr::from_ref(self) as usize).or_insert(self.initial);
            let previous = *value;
            operation(value);
            previous
        })
    }
    pub(super) fn load(&self, _ordering: Ordering) -> usize { self.update(|_| {}) }
    pub(super) fn store(&self, value: usize, _ordering: Ordering) { self.update(|current| *current = value); }
    pub(super) fn fetch_add(&self, value: usize, _ordering: Ordering) -> usize {
        self.update(|current| *current = current.wrapping_add(value))
    }
    pub(super) fn fetch_sub(&self, value: usize, _ordering: Ordering) -> usize {
        self.update(|current| *current = current.wrapping_sub(value))
    }
}

pub(super) fn count(counter: &AtomicUsize) { counter.fetch_add(1, Ordering::Relaxed); }

pub(super) struct Mutex<T> { initial: T }
impl<T: Clone + 'static> Mutex<T> {
    pub(super) const fn new(initial: T) -> Self { Self { initial } }
    pub(super) fn lock(&self) -> std::sync::LockResult<RecordGuard<T>> {
        let key = std::ptr::from_ref(self) as usize;
        let value = RECORDS.with(|records| records.borrow_mut().remove(&key))
            .map(|value| *value.downcast::<T>().expect("measurement record type"))
            .unwrap_or_else(|| self.initial.clone());
        Ok(RecordGuard { key, value: Some(value) })
    }
}

pub(super) struct RecordGuard<T: 'static> { key: usize, value: Option<T> }
impl<T> std::ops::Deref for RecordGuard<T> {
    type Target = T;
    fn deref(&self) -> &T { self.value.as_ref().expect("live record guard") }
}
impl<T> std::ops::DerefMut for RecordGuard<T> {
    fn deref_mut(&mut self) -> &mut T { self.value.as_mut().expect("live record guard") }
}
impl<T: 'static> Drop for RecordGuard<T> {
    fn drop(&mut self) {
        // Keep the owned record even during an expected catch_unwind; production
        // locks likewise recover poisoned contents through into_inner().
        let value = self.value.take().expect("live record guard");
        RECORDS.with(|records| { records.borrow_mut().insert(self.key, Box::new(value)); });
    }
}

#[test]
fn another_test_thread_reset_cannot_erase_live_measurements() {
    use std::sync::{Arc, Barrier};
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    static STAGES: Mutex<Vec<usize>> = Mutex::new(Vec::new());
    let barrier = Arc::new(Barrier::new(2));
    let other_barrier = barrier.clone();
    let other = std::thread::spawn(move || {
        COUNTER.store(1, Ordering::Relaxed);
        STAGES.lock().unwrap().push(1);
        other_barrier.wait();
        other_barrier.wait();
        assert_eq!(COUNTER.fetch_sub(1, Ordering::Relaxed), 1);
        assert_eq!(*STAGES.lock().unwrap(), vec![1]);
    });
    barrier.wait();
    COUNTER.store(0, Ordering::Relaxed);
    STAGES.lock().unwrap().clear();
    barrier.wait();
    other.join().unwrap();
    assert_eq!(COUNTER.load(Ordering::Relaxed), 0);
    assert!(STAGES.lock().unwrap().is_empty());
}
