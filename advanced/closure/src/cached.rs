


struct Cached<T>
where
 T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}
