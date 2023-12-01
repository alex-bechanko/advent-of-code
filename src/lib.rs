mod y2022;

pub type Solution<'a> = &'a dyn Fn(&str) -> Result<String, String>;
pub const YEARS: [ &[(Solution, Solution)]; 1 ] = [
    &y2022::DAYS,
];
