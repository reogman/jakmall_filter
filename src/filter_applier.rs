use crate::{traits::Provider, Filter};

#[derive(Debug)]
pub struct Reason {
    pub filter_requirement: String,
    pub target_value: String,
    pub field: String,
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum FilterResult {
    Next,
    NotPassWithReason(Reason),
}

impl Filter {
    pub fn apply_filter_to<P>(&self, target: P) -> FilterResult
    where
        P: Provider,
    {
        let mut reason: Option<Reason> = None;

        if self.can_single_co != target.get_can_single_co() {
            reason = Some(Reason {
                field: "can_single_co".into(),
                filter_requirement: self.can_single_co.to_string(),
                target_value: target.get_can_single_co().to_string(),
            });
        }

        if self.max_price != target.get_max_price() {
            reason = Some(Reason {
                field: "max_price".into(),
                filter_requirement: self.max_price.to_string(),
                target_value: target.get_max_price().to_string(),
            });
        }

        if self.min_price != target.get_min_price() {
            reason = Some(Reason {
                field: "min_price".into(),
                filter_requirement: self.min_price.to_string(),
                target_value: target.get_min_price().to_string(),
            });
        }

        if self.min_sold != target.get_min_sold() {
            reason = Some(Reason {
                filter_requirement: self.min_sold.to_string(),
                target_value: target.get_min_sold().to_string(),
                field: "min_sold".into(),
            })
        }

        match reason {
            Some(r) => FilterResult::NotPassWithReason(r),
            None => FilterResult::Next,
        }
    }
}
