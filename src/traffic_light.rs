#[allow(dead_code)]
enum TrafficLight{
    Red,
    Yellow,
    Green
}

trait Durative {
    fn get_duration(&self) -> i32;
}

impl Durative for TrafficLight{
    fn get_duration(&self)-> i32{
        match self {
            TrafficLight::Red => {
                return 40;
            },
            TrafficLight::Yellow=> {
                return 10;
            },
            TrafficLight::Green=>{
                return 30;
            },
        };
    }
}

#[test]
fn test_red() {
    let red = TrafficLight::Red;
    assert_eq!(red.get_duration(), 40);
}
#[test]
fn test_yellow() {
    let yellow = TrafficLight::Yellow;
    assert_eq!(yellow.get_duration(), 10);
}
#[test]
fn test_green() {
    let green = TrafficLight::Green;
    assert_eq!(green.get_duration(), 30);
}