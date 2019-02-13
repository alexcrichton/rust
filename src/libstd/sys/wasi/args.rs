use crate::ffi::OsString;
use crate::marker::PhantomData;

pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
    // On wasm these should always be null, so there's nothing for us to do here
}

pub unsafe fn cleanup() {
}

pub fn args() -> Args {
    Args {
        _dont_send_or_sync_me: PhantomData,
    }
}

pub struct Args {
    _dont_send_or_sync_me: PhantomData<*mut ()>,
}

impl Args {
    pub fn inner_debug(&self) -> &[OsString] {
        &[]
    }
}

impl Iterator for Args {
    type Item = OsString;

    fn next(&mut self) -> Option<OsString> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        0
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<OsString> {
        None
    }
}
