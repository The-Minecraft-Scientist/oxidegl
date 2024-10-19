include!(concat!(env!("OUT_DIR"), "/generated.rs"));
pub(crate) struct ConstStrToU16Map<const N: usize> {
    keys: [&'static str; N],
    vals: [u16; N],
}
impl<const N: usize> ConstStrToU16Map<N> {
    pub(crate) const fn get(&self, key: &'static str) -> Option<u16> {
        let mut low = 0;
        let mut high = self.keys.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if compare_strings(self.keys[mid], key) == 0 {
                return Some(self.vals[mid]);
            } else if compare_strings(key, self.keys[mid]) == -1 {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        None
    }
}
const fn min_usize(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
/// Returns: 1 if a > b, 0 if a = b, -1 if a < b
const fn compare_strings(a: &str, b: &str) -> i32 {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let max_idx = min_usize(a.len(), b.len());
    let mut i = 0;
    while i < max_idx {
        if a[i] > b[i] {
            return 1;
        }
        if a[i] < b[i] {
            return -1;
        }
        i += 1;
    }
    if a.len() > b.len() {
        return 1;
    }
    if a.len() < b.len() {
        return -1;
    }
    0
}
