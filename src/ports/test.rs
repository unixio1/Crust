extern crate rand;
use super::PortSelector;
use super::RandomStrategy;
use super::RangeStrategy;

#[test]
fn test_range_port_strategy(){
    let strategy = RangeStrategy{low: 1, high: 10};
    assert_eq!(strategy.get_ports(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_random_port_strategy(){
    let strategy = RandomStrategy{low: 20, high: 100, amount: 50};
    let ports: Vec<u16> = strategy.get_ports();
    assert_eq!(ports.len(), 50);
    let min_value = ports.iter().min();
    let max_value = ports.iter().max();
    match min_value {
        Some(value) => assert!(value >= &20),
        None => assert!(false)
    }
    match max_value {
        Some(value) => assert!(value <= &100),
        None => assert!(false)
    }
    let mut non_shuffeled_ports: Vec<u16> = (20..=100).collect();
    non_shuffeled_ports.truncate(50);
    assert_ne!(ports, non_shuffeled_ports);

}
