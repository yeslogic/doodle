use std::cmp::Ordering;
use anyhow::{anyhow, Result as AResult};


#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum ByteOffset {
    Bytes(usize),
    Bits(usize),
}

impl std::fmt::Display for ByteOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_bytes() {
            (n, Some(k)) => write!(f, "{n}:{k}"),
            (n, None) => write!(f, "{n}"),
        }
    }
}


impl Default for ByteOffset {
    fn default() -> Self {
        ByteOffset::Bytes(0)
    }
}

impl ByteOffset {
    pub(crate) const fn from_bytes(nbytes: usize) -> Self {
        Self::Bytes(nbytes)
    }

    pub(crate) const fn from_bits(nbits: usize) -> Self {
        Self::Bits(nbits)
    }

    pub(crate) fn is_bit_mode(&self) -> bool {
        matches!(self, Self::Bits(..))
    }

    pub(crate) fn increment_assign(&mut self) {
        self.increment_assign_by(1)
    }

    pub(crate) fn increment_by(&self, delta: usize) -> Self {
        match self {
            &ByteOffset::Bytes(n_bytes) => Self::Bytes(n_bytes + delta),
            &ByteOffset::Bits(n_bits) => Self::Bits(n_bits + delta),
        }
    }

    pub(crate) fn increment_assign_by(&mut self, delta: usize) {
        match self {
            ByteOffset::Bytes(n_bytes) => *n_bytes += delta,
            ByteOffset::Bits(n_bits) => *n_bits += delta,
        }
    }

    pub(crate) fn enter_bits_mode(&mut self) -> AResult<()> {
        if let ByteOffset::Bytes(n_bytes) = *self {
            *self = ByteOffset::Bits(n_bytes * 8);
            Ok(())
        } else {
            Err(anyhow!("Cannot enter 'Bits' mode while already in 'Bits' mode"))
        }
    }

    pub(crate) fn escape_bits_mode(&mut self) -> AResult<()> {
        if let ByteOffset::Bits(n_bits) = *self {
            let floor = n_bits / 8;
            if n_bits % 8 != 0 {
                *self = ByteOffset::Bytes(floor + 1);
            } else {
                *self = ByteOffset::Bytes(floor);
            }
            Ok(())
        } else {
            Err(anyhow!("Cannot escape 'Bits' mode while not currently in 'Bits' mode"))
        }
    }

    pub(crate) fn as_bits(&self) -> usize {
        match self {
            ByteOffset::Bytes(n) => *n * 8,
            ByteOffset::Bits(n) => *n,
        }
    }

    pub(crate) fn as_bytes(&self) -> (usize, Option<usize>) {
        match self {
            ByteOffset::Bytes(n) => (*n, None),
            ByteOffset::Bits(n) => (*n / 8, Some(*n % 8)),
        }
    }
}

impl PartialOrd for ByteOffset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let partial = self.as_bits().cmp(&other.as_bits());
        match partial {
            // yield None instead of Some(Equal) if same bit-level offset but different modes
            Ordering::Equal if self.is_bit_mode() ^ other.is_bit_mode() => None,
            _ => Some(partial),
        }
    }
}

pub(crate) struct BufferOffset {
    current_offset: ByteOffset, // the 'true' value of the offset, reinterpreted according to the other fields
    checkpoints: PriorityStack<ByteOffset>, // as a FILO stack, with each top >= the last
    limits: Vec<ByteOffset>, // as a FILO stack, with each top <= the last.
    max_offset: ByteOffset,
}

impl BufferOffset {
    /// Takes the maximum legal value for the offset (equal to the buffer's total length in bytes)
    /// and returns a new BufferOffset starting from 0.
    pub(crate) fn new(max_offset: ByteOffset) -> Self {
        Self {
            current_offset: ByteOffset::default(),
            checkpoints: PriorityStack::new(),
            limits: Vec::new(),
            max_offset,
        }
    }

    pub(crate) fn can_increment(&self, delta: usize) -> bool {
        let lim = <Vec<ByteOffset> as CopyStack>::peek_or(&self.limits, self.max_offset);
        let after_increment = self.current_offset.increment_by(delta);
        !(after_increment > lim)
    }

    pub(crate) fn try_increment(&mut self, delta: usize) -> AResult<()> {
        let lim = <Vec<ByteOffset> as CopyStack>::peek_or(&self.limits, self.max_offset);
        let after_increment = self.current_offset.increment_by(delta);
        if !(after_increment > lim) {
            self.current_offset.increment_assign_by(delta);
            Ok(())
        } else {
            Err(anyhow!("Cannot increment current offset {} by {} without violating limit {}", self.current_offset, delta, lim))
        }
    }

    pub(crate) fn push_limit(&mut self, limit: ByteOffset) -> AResult<()> {
        let f = |o_lim: Option<ByteOffset>, new_lim: ByteOffset| -> Option<anyhow::Error> {
            let old_lim = o_lim.unwrap_or(self.max_offset);
            (new_lim > old_lim).then(|| anyhow!("new limit {} exceeds previous limit {}", new_lim, old_lim))
        };
        <Vec<ByteOffset> as CopyStack>::push_cond(&mut self.limits, limit, f)
    }

    pub(crate) fn escape_limit(&mut self) -> AResult<ByteOffset> {
        if let Some(offs) = self.limits.pop() {
            if self.current_offset > offs {
                return Err(anyhow!("Current offset exceeds limit being escaped: {} > {}", self.current_offset, offs));
            }
            self.current_offset = offs;
            Ok(offs)
        } else {
            Err(anyhow!("No enforced limit to escape"))
        }
    }
}

pub(crate) struct PriorityStack<T> {
    store: Vec<(T, bool)>,
}

impl<T> PriorityStack<T> {
    pub(crate) const fn new() -> Self {
        Self { store: Vec::new() }
    }

    pub(crate) fn pop_any(&mut self) -> Option<T> {
        let (ret, _) = self.store.pop()?;
        Some(ret)
    }

    pub(crate) fn pop_marked(&mut self) -> Option<T> {
        while let (ret, is_marked) = self.store.pop()? {
            if is_marked {
                return Some(ret);
            }
        }
        None
    }
}

pub trait CopyStack {
    type Elem: Copy;

    fn peek(&self) -> Option<Self::Elem>;

    fn peek_or(&self, default: Self::Elem) -> Self::Elem {
        self.peek().unwrap_or(default)
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Elem>;

    fn pop(&mut self) -> Option<Self::Elem>;

    fn pop_or(&mut self, default: Self::Elem) -> Self::Elem {
        self.pop().unwrap_or(default)
    }

    fn push(&mut self, item: Self::Elem);

    /// Pushes an element if and only if the provided validation function returns `None` when called over
    /// the current stack-top and the item prospectively being pushed.
    ///
    /// If the validation function returned `Some(e)` instead, Does nothing and returns `Err(e)`.
    fn push_cond<E, F>(&mut self, item: Self::Elem, f: F) -> Result<(), E> where
        F: Fn(Option<Self::Elem>, Self::Elem) -> Option<E>
    {
        match f(self.peek(), item) {
            None => Ok(self.push(item)),
            Some(err) => Err(err),
        }
    }

    fn size(&self) -> usize;

    fn is_empty(&self) -> bool;
}

impl<T: Copy> CopyStack for Vec<T> {
    type Elem = T;

    fn peek(&self) -> Option<Self::Elem> {
        self.last().copied()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Elem> {
        self.last_mut()
    }

    fn pop(&mut self) -> Option<Self::Elem> {
        Vec::pop(self)
    }

    fn push(&mut self, item: Self::Elem) {
        Vec::push(self, item)
    }

    fn size(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}
