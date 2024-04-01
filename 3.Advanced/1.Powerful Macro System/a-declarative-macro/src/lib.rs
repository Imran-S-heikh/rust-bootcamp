#[macro_export]
macro_rules! map {
    ($key:ty,$val:ty) => {{
        let map = HashMap::<$key, $val>::new();

        map
    }};
}
