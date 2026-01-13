use crate::MinmaxCompare;
use std::ops::Deref;
use std::sync::Arc;

pub trait MinmaxComparator<V: PartialOrd> {
    fn exchange_if_less_than(&mut self, another: &Arc<V>);
    fn exchange_if_greater_than(&mut self, another: &Arc<V>);

    fn exchange_if(&mut self, another: &Arc<V>, compare: &MinmaxCompare) {
        match compare {
            MinmaxCompare::Less => self.exchange_if_greater_than(another),
            MinmaxCompare::Greater => self.exchange_if_less_than(another),
        }
    }
}

impl<V: PartialOrd> MinmaxComparator<V> for Option<Arc<V>> {
    /// returns min value between self and another
    fn exchange_if_less_than(&mut self, another: &Arc<V>) {
        if let Some(one) = self {
            if one.deref() > another {
            } else {
                *self = Some(another.clone())
            }
        } else {
            *self = Some(another.clone())
        }
    }

    /// returns min value between self and another
    fn exchange_if_greater_than(&mut self, another: &Arc<V>) {
        if let Some(one) = self {
            if one.deref() < another {
            } else {
                *self = Some(another.clone())
            }
        } else {
            *self = Some(another.clone())
        }
    }
}
