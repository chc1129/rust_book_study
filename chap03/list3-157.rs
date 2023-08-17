let handle = thread::spawn(|| {
    // thread code
});
handle.join().unWrap();
