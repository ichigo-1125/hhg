/*

    Deque

    ----------------------------------------------------------------------------

    # 目次

    - Deque

    - コンストラクタ

*/

#![doc = include_str!("../../doc/collections/deque/deque.md")]

use std::collections::VecDeque;


//------------------------------------------------------------------------------
//  Deque
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct Deque<T>
{
    pub deq: VecDeque<T>,
}


//------------------------------------------------------------------------------
//  コンストラクタ
//------------------------------------------------------------------------------
impl<T> Deque<T>
{
    //--------------------------------------------------------------------------
    //  new
    //
    //  空のDequeを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/new.md")]
    #[inline]
    #[must_use]
    pub fn new() -> Self
    {
        Self { deq: VecDeque::new() }
    }

    //--------------------------------------------------------------------------
    //  with_capacity
    //
    //  容量を指定して空のVectorを生成する。
    //--------------------------------------------------------------------------
    #[doc = include_str!("../../doc/collections/deque/with_capacity.md")]
    #[inline]
    #[must_use]
    pub fn with_capacity( capacity: usize ) -> Self
    {
        Self { deq: VecDeque::with_capacity(capacity) }
    }
}


//------------------------------------------------------------------------------
//  macro
//------------------------------------------------------------------------------
#[macro_export]
macro_rules! deque
{
    () =>
    {
        Deque { deq: ::std::collections::VecDeque::new() }
    };

    ($elem: expr; $n: expr) =>
    {
        let mut deq = ::std::collections::VecDeque::new();
        deq.resize_with($n, || $elem);
        Deque { deq }
    };

    ( $($elem: expr),+ $(,)? ) =>
    {
        const CAP: usize = $crate::count!($($elem),*);
        let mut deq = ::std::collections::VecDeque::with_capacity(CAP);
        $(
            deq.push_back($elem);
        )+
        Deque { deq }
    };
}
