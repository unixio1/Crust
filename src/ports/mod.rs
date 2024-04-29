use rand::thread_rng;
use rand::seq::SliceRandom;


pub trait PortSelector{
    fn get_ports(&self) -> Vec<u16>;
}

pub struct RangeStrategy{
    pub low: u16,
    pub high: u16
}

impl PortSelector for RangeStrategy{
    fn get_ports(&self) -> Vec<u16> {
        return (self.low..=self.high).collect();
    }
}

pub struct RandomStrategy{
    pub low: u16,
    pub high: u16,
    pub amount: usize,
}

impl PortSelector for RandomStrategy{
    fn get_ports(&self) -> Vec<u16> {
        let mut ports: Vec<u16> = (self.low..=self.high).collect();
        ports.shuffle(&mut thread_rng());
        ports.truncate(self.amount);
        return ports;
    }
}

pub struct ManualStrategy{
    pub selected_ports: Vec<u16>
}

impl PortSelector for ManualStrategy{
    fn get_ports(&self) -> Vec<u16> {
        return self.selected_ports.to_vec();
    }
}

pub struct SignificanceStrategy{

}


#[cfg(test)]
pub mod test;
