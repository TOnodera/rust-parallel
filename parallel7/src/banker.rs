use std::sync::{Arc, Mutex};
pub struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],
    allocation: [[usize; NRES]; NTH],
    max: [[usize; NRES]; NTH],
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Self {
            available,
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

    // 現状の状態が安全かを調査
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            // すべてのスレッドi,jにおいて
            // finish[i] == false && work[j] >= (self.max[i][j] - self.allocation[i][j])
            // を満たすようなスレッドを見つける
            let mut found = false;
            let mut num_true = 0;
            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
                let is_available = work.iter().zip(need).all(|(w, n)| *w >= n);

                if is_available {
                    found = true;
                    finish[i] = true;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a;
                    }
                    break;
                }
            }
            if num_true == NTH {
                return true;
            }

            if !found {
                break;
            }
        }
        false
    }

    // id番目のスレッドがresourceを１つ確保
    fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH
            || resource >= NRES
            || self.available[resource] == 0
            || self.max[id][resource] == self.allocation[id][resource]
        {
            return false;
        }

        // リソースの確保を試みる
        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            // リソース確保に失敗したため状態を復元
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    // id番目のスレッドがresourceを１つ解放
    fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }
        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }
}

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Self {
            resource: Arc::new(Mutex::new(Resource::new(available, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        r.take(id, resource)
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource)
    }
}
