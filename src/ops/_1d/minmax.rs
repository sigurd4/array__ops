use super::{DivideAndConquer, Reduce};

#[const_trait]
pub trait Minmax<T, const N: usize>: Reduce<T, N>
{
    fn max(self) -> Option<T>
    where
        T: Ord;
    fn min(self) -> Option<T>
    where
        T: Ord;
    async fn max_async(self) -> Option<T>
    where
        T: Ord;
    async fn min_async(self) -> Option<T>
    where
        T: Ord;
        
    fn first_max(self) -> Option<T>
    where
        T: PartialOrd<T>;
        
    fn first_min(self) -> Option<T>
    where
        T: PartialOrd<T>;
}

impl<T, const N: usize> Minmax<T, N> for [T; N]
{
    fn max(self) -> Option<T>
    where
        T: Ord
    {
        self.reduce(T::max)
    }
    fn min(self) -> Option<T>
    where
        T: Ord
    {
        self.reduce(T::min)
    }
    async fn max_async(self) -> Option<T>
    where
        T: Ord
    {
        self.divide_and_conquer_async(async |x, y| x.max(y)).await
    }
    async fn min_async(self) -> Option<T>
    where
        T: Ord
    {
        self.divide_and_conquer_async(async |x, y| x.min(y)).await
    }
    
    fn first_max(self) -> Option<T>
    where
        T: PartialOrd<T>
    {
        self.reduce(|a, b| if a >= b {a} else {b})
    }
    fn first_min(self) -> Option<T>
    where
        T: PartialOrd<T>
    {
        self.reduce(|a, b| if a <= b {a} else {b})
    }
}