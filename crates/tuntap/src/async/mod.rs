mod reactor;

#[cfg(test)]
mod test {
    use futures::executor::LocalPool;

    #[test]
    fn test_future() {
        let mut local = LocalPool::new();
        let out = local.run_until(async {
            std::thread::current().id()
        });
        assert_eq!(std::thread::current().id(), out);
    }
}