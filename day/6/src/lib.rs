#[derive(Debug)]
pub struct Race {
    pub duration: u64,
    pub record_distance: u64,
}

impl Race {
    pub fn get_duration(&self) -> u64 {
        self.duration
    }

    pub fn get_record_distance(&self) -> u64 {
        self.record_distance
    }

    pub fn get_distance(&self, charge_time: u64) -> u64 {
        if charge_time == 0 || charge_time == self.duration {
            return 0;
        }

        let time_to_move = self.duration - charge_time;
        let speed = charge_time;

        speed * time_to_move
    }

    pub fn get_number_of_ways_to_win(&self) -> u64 {
        let mut n: u64 = 0;

        for i in 1..=self.duration {
            if self.get_distance(i) > self.record_distance {
                n += 1;
            }
        }

        n
    }
}
