use crate::check;
use rand::random_bool;

check!(
    Ident: RandomCheck,
    Name: "Random",
    Description: "Check which will randomly resolve to Up or Down depending according to a provided probability.",

    Fields: [
        likelihood:
            Name: "Likelihood",
            Description: "Likelihood out of 100% that the check will resolve to Up.",
            Default: CheckFieldValue::Percentage(50.0f32),
            Type: f32,
    ]

    fn check(&self) -> Result<CheckResult, CheckError> {
        let likelihood: f32 = (self.likelihood / 100.0).clamp(0.0, 1.0);
        println!("hello");
        Ok(CheckResult {
            status: match random_bool(likelihood as f64) {
                true => CheckStatus::Up,
                false => CheckStatus::Down,
            },
            message: None,
        })
    }
);
