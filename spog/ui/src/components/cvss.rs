use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::utils::cvss::{Cvss, Severity};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CvssScoreProperties {
    pub cvss: Cvss,
}

#[function_component(CvssScore)]
pub fn cvss_information(props: &CvssScoreProperties) -> Html {
    let label = format!("{}", props.cvss.score);

    let (color, outline) = match props.cvss.to_severity() {
        Severity::None => (Color::Grey, true),
        Severity::Low => (Color::Orange, true),
        Severity::Medium => (Color::Orange, false),
        Severity::High => (Color::Red, false),
        Severity::Critical => (Color::Purple, false),
    };

    html!(
        <Label {label} {color} {outline}/>
    )
}
