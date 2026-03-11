use crate::chain::types::TxInfo;

/// 트랜잭션 필터
#[derive(Debug, Clone)]
pub struct TxFilter {
    pub min_value_eth: Option<f64>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub active: bool,
}

impl Default for TxFilter {
    fn default() -> Self {
        Self {
            min_value_eth: None,
            from_address: None,
            to_address: None,
            active: false,
        }
    }
}

impl TxFilter {
    /// 필터 조건에 맞는지 확인
    pub fn matches(&self, tx: &TxInfo) -> bool {
        if !self.active {
            return true;
        }

        if let Some(min) = self.min_value_eth {
            if tx.value_eth < min {
                return false;
            }
        }

        if let Some(ref addr) = self.from_address {
            if !tx.from_full.eq_ignore_ascii_case(addr) {
                return false;
            }
        }

        if let Some(ref addr) = self.to_address {
            if let Some(ref to) = tx.to_full {
                if !to.eq_ignore_ascii_case(addr) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    /// 필터 활성/비활성 토글
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }

    /// 필터 조건이 설정되어 있는지 확인
    pub fn has_conditions(&self) -> bool {
        self.min_value_eth.is_some()
            || self.from_address.is_some()
            || self.to_address.is_some()
    }

    /// CLI 인자로 필터 설정
    pub fn from_cli(
        min_value: Option<f64>,
        from: Option<String>,
        to: Option<String>,
    ) -> Self {
        let has_any = min_value.is_some() || from.is_some() || to.is_some();
        Self {
            min_value_eth: min_value,
            from_address: from,
            to_address: to,
            active: has_any,
        }
    }
}
