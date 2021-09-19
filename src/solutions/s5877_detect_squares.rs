use std::collections::HashMap;

#[derive(Default)]
struct DetectSquares {
    map: HashMap<i32, HashMap<i32, i32>>,
}

#[allow(dead_code)]
impl DetectSquares {
    fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, point: Vec<i32>) {
        assert!(point.len() == 2);
        let (x, y) = (point[0], point[1]);
        *self.map.entry(x).or_default().entry(y).or_default() += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        assert!(point.len() == 2);

        let (x, y) = (point[0], point[1]);
        let mut result = 0;
        for (&y1, &nxy1) in self.map.get(&x).unwrap_or(&HashMap::default()).into_iter() {
            let side = (y - y1).abs();
            if side > 0 {
                result += nxy1 * self.get(x + side, y) * self.get(x + side, y1);
                result += nxy1 * self.get(x - side, y) * self.get(x - side, y1);
            }
        }

        result
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        match self.map.get(&x) {
            None => 0,
            Some(row) => match row.get(&y) {
                None => 0,
                Some(result) => *result,
            },
        }
    }
}
