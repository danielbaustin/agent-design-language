use core::future::Future;

fn forbidden<T: Future>(_value: T) {}
