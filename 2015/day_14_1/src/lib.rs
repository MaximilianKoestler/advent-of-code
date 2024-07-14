#[derive(Debug, PartialEq, Eq)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,
    pub fly_time: u32,
    pub rest_time: u32,
}

mod parsers {
    use nom::{
        bytes::complete::tag, character::complete::alpha1, combinator::map, sequence::terminated,
        sequence::tuple, IResult,
    };

    use super::Reindeer;

    fn name(input: &str) -> IResult<&str, String> {
        map(alpha1, |name: &str| name.to_string())(input)
    }

    fn speed(input: &str) -> IResult<&str, u32> {
        terminated(nom::character::complete::u32, tag(" km/s"))(input)
    }

    fn time(input: &str) -> IResult<&str, u32> {
        terminated(nom::character::complete::u32, tag(" seconds"))(input)
    }

    pub fn reindeer(input: &str) -> IResult<&str, Reindeer> {
        let (input, (name, _, speed, _, fly_time, _, rest_time)) = tuple((
            name,
            tag(" can fly "),
            speed,
            tag(" for "),
            time,
            tag(", but then must rest for "),
            time,
        ))(input)?;

        Ok((
            input,
            Reindeer {
                name,
                speed,
                fly_time,
                rest_time,
            },
        ))
    }
}

impl<'a> TryFrom<&'a str> for Reindeer {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        parsers::reindeer(input).map(|(_, c)| c)
    }
}

pub trait SimulateUntil {
    fn simulate(&self, time: u32) -> u32;
}

impl SimulateUntil for Reindeer {
    fn simulate(&self, time: u32) -> u32 {
        let period = self.fly_time + self.rest_time;
        let full_periods = time / period;
        let remaining_time = time - (period * full_periods);
        let remaining_fly_time = remaining_time.min(self.fly_time);

        let total_fly_time = full_periods * self.fly_time + remaining_fly_time;
        total_fly_time * self.speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reindeer() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.";
        let expected = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let actual = Reindeer::try_from(input).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simulate() {
        let reindeer = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        assert_eq!(reindeer.simulate(1000), 1120);

        let reindeer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };
        assert_eq!(reindeer.simulate(1000), 1056);
    }
}
